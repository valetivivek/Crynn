use crate::{EmailHeader, EmailBody};
use anyhow::Result;
use sqlx::{SqlitePool, Row};
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::sync::RwLock;

pub struct EmailCache {
    pool: SqlitePool,
    max_size_bytes: usize,
    current_size: AtomicUsize,
    headers_cache: RwLock<Vec<EmailHeader>>,
}

impl EmailCache {
    pub async fn new(max_size_mb: usize, pool: SqlitePool) -> Result<Self> {
        let max_size_bytes = max_size_mb * 1024 * 1024;

        // Initialize cache tables
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS email_headers (
                uid INTEGER PRIMARY KEY,
                subject TEXT NOT NULL,
                from_addr TEXT NOT NULL,
                to_addr TEXT NOT NULL,
                date TEXT NOT NULL,
                size INTEGER NOT NULL,
                flags TEXT NOT NULL,
                folder TEXT NOT NULL,
                cached_at TEXT NOT NULL
            )
            "#,
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS email_bodies (
                uid INTEGER PRIMARY KEY,
                content TEXT NOT NULL,
                content_type TEXT NOT NULL,
                cached_at TEXT NOT NULL,
                FOREIGN KEY (uid) REFERENCES email_headers (uid)
            )
            "#,
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS attachments (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                uid INTEGER NOT NULL,
                filename TEXT NOT NULL,
                content_type TEXT NOT NULL,
                size INTEGER NOT NULL,
                data BLOB NOT NULL,
                cached_at TEXT NOT NULL,
                FOREIGN KEY (uid) REFERENCES email_headers (uid)
            )
            "#,
        )
        .execute(&pool)
        .await?;

        Ok(Self {
            pool,
            max_size_bytes,
            current_size: AtomicUsize::new(0),
            headers_cache: RwLock::new(Vec::new()),
        })
    }

    pub async fn store_headers(&self, headers: &[EmailHeader]) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        for header in headers {
            sqlx::query(
                r#"
                INSERT OR REPLACE INTO email_headers 
                (uid, subject, from_addr, to_addr, date, size, flags, folder, cached_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(header.uid)
            .bind(&header.subject)
            .bind(&header.from)
            .bind(&header.to)
            .bind(header.date.to_rfc3339())
            .bind(header.size)
            .bind(serde_json::to_string(&header.flags)?)
            .bind(&header.folder)
            .bind(chrono::Utc::now().to_rfc3339())
            .execute(&mut *tx)
            .await?;

            self.current_size.fetch_add(header.size, Ordering::Relaxed);
        }

        tx.commit().await?;

        // Update in-memory cache
        {
            let mut cache = self.headers_cache.write().await;
            cache.extend_from_slice(headers);
        }

        // Enforce size limits
        self.enforce_size_limits().await?;

        Ok(())
    }

    pub async fn store_body(&self, body: &EmailBody) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        sqlx::query(
            r#"
            INSERT OR REPLACE INTO email_bodies 
            (uid, content, content_type, cached_at)
            VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(body.uid)
        .bind(&body.content)
        .bind(&body.content_type)
        .bind(chrono::Utc::now().to_rfc3339())
        .execute(&mut *tx)
        .await?;

        // Store attachments
        for attachment in &body.attachments {
            sqlx::query(
                r#"
                INSERT OR REPLACE INTO attachments 
                (uid, filename, content_type, size, data, cached_at)
                VALUES (?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(body.uid)
            .bind(&attachment.filename)
            .bind(&attachment.content_type)
            .bind(attachment.size)
            .bind(&attachment.data)
            .bind(chrono::Utc::now().to_rfc3339())
            .execute(&mut *tx)
            .await?;

            self.current_size.fetch_add(attachment.size, Ordering::Relaxed);
        }

        tx.commit().await?;

        // Enforce size limits
        self.enforce_size_limits().await?;

        Ok(())
    }

    pub async fn get_body(&self, uid: u32) -> Result<Option<EmailBody>> {
        let row = sqlx::query(
            r#"
            SELECT uid, content, content_type FROM email_bodies WHERE uid = ?
            "#,
        )
        .bind(uid)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            let uid: u32 = row.get("uid");
            let content: String = row.get("content");
            let content_type: String = row.get("content_type");

            // Fetch attachments
            let attachment_rows = sqlx::query(
                r#"
                SELECT filename, content_type, size, data FROM attachments WHERE uid = ?
                "#,
            )
            .bind(uid)
            .fetch_all(&self.pool)
            .await?;

            let mut attachments = Vec::new();
            for row in attachment_rows {
                attachments.push(crate::Attachment {
                    filename: row.get("filename"),
                    content_type: row.get("content_type"),
                    size: row.get("size"),
                    data: row.get("data"),
                });
            }

            Ok(Some(EmailBody {
                uid,
                content,
                content_type,
                attachments,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn search_headers(&self, query: &str) -> Result<Vec<EmailHeader>> {
        let rows = sqlx::query(
            r#"
            SELECT uid, subject, from_addr, to_addr, date, size, flags, folder
            FROM email_headers 
            WHERE subject LIKE ? OR from_addr LIKE ? OR to_addr LIKE ?
            ORDER BY date DESC
            "#,
        )
        .bind(format!("%{}%", query))
        .bind(format!("%{}%", query))
        .bind(format!("%{}%", query))
        .fetch_all(&self.pool)
        .await?;

        let mut headers = Vec::new();
        for row in rows {
            let flags_json: String = row.get("flags");
            let flags: Vec<String> = serde_json::from_str(&flags_json)?;
            let date_str: String = row.get("date");
            let date = chrono::DateTime::parse_from_rfc3339(&date_str)?.with_timezone(&chrono::Utc);

            headers.push(EmailHeader {
                uid: row.get("uid"),
                subject: row.get("subject"),
                from: row.get("from_addr"),
                to: row.get("to_addr"),
                date,
                size: row.get("size"),
                flags,
                folder: row.get("folder"),
            });
        }

        Ok(headers)
    }

    pub async fn get_memory_usage(&self) -> Result<usize> {
        Ok(self.current_size.load(Ordering::Relaxed))
    }

    pub async fn cleanup(&self) -> Result<()> {
        // Remove oldest cached items when over limit
        let current_size = self.current_size.load(Ordering::Relaxed);
        if current_size > self.max_size_bytes {
            let excess = current_size - self.max_size_bytes;
            
            // Remove oldest headers first
            sqlx::query(
                r#"
                DELETE FROM email_headers 
                WHERE uid IN (
                    SELECT uid FROM email_headers 
                    ORDER BY cached_at ASC 
                    LIMIT ?
                )
                "#,
            )
            .bind(excess / 1024) // Rough estimate of items to remove
            .execute(&self.pool)
            .await?;

            // Reset size counter (will be recalculated on next access)
            self.current_size.store(0, Ordering::Relaxed);
        }

        Ok(())
    }

    async fn enforce_size_limits(&self) -> Result<()> {
        let current_size = self.current_size.load(Ordering::Relaxed);
        if current_size > self.max_size_bytes {
            self.cleanup().await?;
        }
        Ok(())
    }
}
