use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, Row};
use std::path::PathBuf;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

pub mod cache;
pub mod cookies;
pub mod history;
pub mod bookmarks;

use cache::CacheManager;
use cookies::CookieManager;
use history::HistoryManager;
use bookmarks::BookmarkManager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub data_dir: PathBuf,
    pub cache_size_mb: usize,
    pub max_history_items: usize,
    pub max_bookmarks: usize,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            data_dir: dirs::data_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("crynn"),
            cache_size_mb: 100,
            max_history_items: 10000,
            max_bookmarks: 1000,
        }
    }
}

pub struct StorageManager {
    config: StorageConfig,
    pool: SqlitePool,
    cache: CacheManager,
    cookies: CookieManager,
    history: HistoryManager,
    bookmarks: BookmarkManager,
}

impl StorageManager {
    pub async fn new(config: StorageConfig) -> Result<Self> {
        // Create data directory
        std::fs::create_dir_all(&config.data_dir)?;

        // Initialize database
        let db_path = config.data_dir.join("crynn.db");
        let pool = SqlitePool::connect(&format!("sqlite:{}", db_path.display())).await?;

        // Run migrations
        sqlx::migrate!("./migrations").run(&pool).await?;

        // Initialize managers
        let cache = CacheManager::new(config.cache_size_mb, pool.clone()).await?;
        let cookies = CookieManager::new(pool.clone()).await?;
        let history = HistoryManager::new(config.max_history_items, pool.clone()).await?;
        let bookmarks = BookmarkManager::new(config.max_bookmarks, pool.clone()).await?;

        Ok(Self {
            config,
            pool,
            cache,
            cookies,
            history,
            bookmarks,
        })
    }

    pub fn cache(&self) -> &CacheManager {
        &self.cache
    }

    pub fn cookies(&self) -> &CookieManager {
        &self.cookies
    }

    pub fn history(&self) -> &HistoryManager {
        &self.history
    }

    pub fn bookmarks(&self) -> &BookmarkManager {
        &self.bookmarks
    }

    pub async fn get_storage_stats(&self) -> Result<StorageStats> {
        let cache_size = self.cache.get_size().await?;
        let history_count = self.history.count().await?;
        let bookmark_count = self.bookmarks.count().await?;
        let cookie_count = self.cookies.count().await?;

        Ok(StorageStats {
            cache_size_mb: cache_size / (1024 * 1024),
            history_items: history_count,
            bookmarks: bookmark_count,
            cookies: cookie_count,
            total_size_mb: self.get_total_size().await?,
        })
    }

    async fn get_total_size(&self) -> Result<usize> {
        // Calculate total storage size
        let mut total = 0;
        
        // Database size
        let db_path = self.config.data_dir.join("crynn.db");
        if db_path.exists() {
            total += std::fs::metadata(&db_path)?.len() as usize;
        }

        // Cache directory size
        let cache_dir = self.config.data_dir.join("cache");
        if cache_dir.exists() {
            total += self.dir_size(&cache_dir).await?;
        }

        Ok(total)
    }

    async fn dir_size(&self, path: &PathBuf) -> Result<usize> {
        let mut total = 0;
        let mut entries = tokio::fs::read_dir(path).await?;
        
        while let Some(entry) = entries.next_entry().await? {
            let metadata = entry.metadata().await?;
            if metadata.is_file() {
                total += metadata.len() as usize;
            } else if metadata.is_dir() {
                total += self.dir_size(&entry.path()).await?;
            }
        }
        
        Ok(total)
    }

    pub async fn cleanup(&self) -> Result<()> {
        self.cache.cleanup().await?;
        self.history.cleanup().await?;
        self.cookies.cleanup().await?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageStats {
    pub cache_size_mb: usize,
    pub history_items: usize,
    pub bookmarks: usize,
    pub cookies: usize,
    pub total_size_mb: usize,
}

// Cache management
pub mod cache {
    use super::*;
    use std::collections::HashMap;
    use std::sync::atomic::{AtomicUsize, Ordering};

    pub struct CacheManager {
        pool: SqlitePool,
        max_size_bytes: usize,
        current_size: AtomicUsize,
        cache_dir: PathBuf,
    }

    impl CacheManager {
        pub async fn new(max_size_mb: usize, pool: SqlitePool) -> Result<Self> {
            let max_size_bytes = max_size_mb * 1024 * 1024;
            let cache_dir = dirs::cache_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("crynn");

            std::fs::create_dir_all(&cache_dir)?;

            // Initialize cache table
            sqlx::query(
                r#"
                CREATE TABLE IF NOT EXISTS cache_entries (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    url TEXT UNIQUE NOT NULL,
                    content_type TEXT NOT NULL,
                    file_path TEXT NOT NULL,
                    size INTEGER NOT NULL,
                    created_at TEXT NOT NULL,
                    accessed_at TEXT NOT NULL,
                    expires_at TEXT
                )
                "#,
            )
            .execute(&pool)
            .await?;

            Ok(Self {
                pool,
                max_size_bytes,
                current_size: AtomicUsize::new(0),
                cache_dir,
            })
        }

        pub async fn get(&self, url: &str) -> Result<Option<CacheEntry>> {
            let row = sqlx::query(
                r#"
                SELECT url, content_type, file_path, size, created_at, accessed_at, expires_at
                FROM cache_entries WHERE url = ?
                "#,
            )
            .bind(url)
            .fetch_optional(&self.pool)
            .await?;

            if let Some(row) = row {
                let file_path: String = row.get("file_path");
                let content = tokio::fs::read(&file_path).await?;
                
                // Update access time
                sqlx::query(
                    r#"
                    UPDATE cache_entries SET accessed_at = ? WHERE url = ?
                    "#,
                )
                .bind(chrono::Utc::now().to_rfc3339())
                .bind(url)
                .execute(&self.pool)
                .await?;

                Ok(Some(CacheEntry {
                    url: row.get("url"),
                    content_type: row.get("content_type"),
                    content,
                    created_at: chrono::DateTime::parse_from_rfc3339(row.get::<String, _>("created_at"))?.with_timezone(&Utc),
                    expires_at: row.get::<Option<String>, _>("expires_at").map(|s| chrono::DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc)),
                }))
            } else {
                Ok(None)
            }
        }

        pub async fn put(&self, url: &str, content_type: &str, content: &[u8], expires_at: Option<DateTime<Utc>>) -> Result<()> {
            let file_path = self.cache_dir.join(format!("{:x}", md5::compute(url)));
            tokio::fs::write(&file_path, content).await?;

            sqlx::query(
                r#"
                INSERT OR REPLACE INTO cache_entries 
                (url, content_type, file_path, size, created_at, accessed_at, expires_at)
                VALUES (?, ?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(url)
            .bind(content_type)
            .bind(file_path.to_string_lossy())
            .bind(content.len())
            .bind(chrono::Utc::now().to_rfc3339())
            .bind(chrono::Utc::now().to_rfc3339())
            .bind(expires_at.map(|dt| dt.to_rfc3339()))
            .execute(&self.pool)
            .await?;

            self.current_size.fetch_add(content.len(), Ordering::Relaxed);
            self.enforce_size_limits().await?;

            Ok(())
        }

        pub async fn get_size(&self) -> Result<usize> {
            Ok(self.current_size.load(Ordering::Relaxed))
        }

        pub async fn cleanup(&self) -> Result<()> {
            // Remove expired entries
            sqlx::query(
                r#"
                DELETE FROM cache_entries WHERE expires_at < ?
                "#,
            )
            .bind(chrono::Utc::now().to_rfc3339())
            .execute(&self.pool)
            .await?;

            // Remove oldest entries if over limit
            self.enforce_size_limits().await?;

            Ok(())
        }

        async fn enforce_size_limits(&self) -> Result<()> {
            let current_size = self.current_size.load(Ordering::Relaxed);
            if current_size > self.max_size_bytes {
                let excess = current_size - self.max_size_bytes;
                
                // Remove oldest entries
                let rows = sqlx::query(
                    r#"
                    SELECT url, file_path FROM cache_entries 
                    ORDER BY accessed_at ASC LIMIT ?
                    "#,
                )
                .bind(excess / 1024) // Rough estimate
                .fetch_all(&self.pool)
                .await?;

                for row in rows {
                    let url: String = row.get("url");
                    let file_path: String = row.get("file_path");
                    
                    // Delete file
                    let _ = tokio::fs::remove_file(&file_path).await;
                    
                    // Delete database entry
                    sqlx::query("DELETE FROM cache_entries WHERE url = ?")
                        .bind(&url)
                        .execute(&self.pool)
                        .await?;
                }

                self.current_size.store(0, Ordering::Relaxed);
            }

            Ok(())
        }
    }

    #[derive(Debug, Clone)]
    pub struct CacheEntry {
        pub url: String,
        pub content_type: String,
        pub content: Vec<u8>,
        pub created_at: DateTime<Utc>,
        pub expires_at: Option<DateTime<Utc>>,
    }
}

// Include other modules
mod cookies {
    use super::*;

    pub struct CookieManager {
        pool: SqlitePool,
    }

    impl CookieManager {
        pub async fn new(pool: SqlitePool) -> Result<Self> {
            sqlx::query(
                r#"
                CREATE TABLE IF NOT EXISTS cookies (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL,
                    value TEXT NOT NULL,
                    domain TEXT NOT NULL,
                    path TEXT NOT NULL,
                    expires_at TEXT,
                    secure BOOLEAN NOT NULL,
                    http_only BOOLEAN NOT NULL,
                    created_at TEXT NOT NULL
                )
                "#,
            )
            .execute(&pool)
            .await?;

            Ok(Self { pool })
        }

        pub async fn count(&self) -> Result<usize> {
            let row = sqlx::query("SELECT COUNT(*) as count FROM cookies")
                .fetch_one(&self.pool)
                .await?;
            Ok(row.get::<i64, _>("count") as usize)
        }

        pub async fn cleanup(&self) -> Result<()> {
            sqlx::query("DELETE FROM cookies WHERE expires_at < ?")
                .bind(chrono::Utc::now().to_rfc3339())
                .execute(&self.pool)
                .await?;
            Ok(())
        }
    }
}

mod history {
    use super::*;

    pub struct HistoryManager {
        pool: SqlitePool,
        max_items: usize,
    }

    impl HistoryManager {
        pub async fn new(max_items: usize, pool: SqlitePool) -> Result<Self> {
            sqlx::query(
                r#"
                CREATE TABLE IF NOT EXISTS history (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    url TEXT NOT NULL,
                    title TEXT,
                    visit_count INTEGER DEFAULT 1,
                    last_visit TEXT NOT NULL,
                    created_at TEXT NOT NULL
                )
                "#,
            )
            .execute(&pool)
            .await?;

            Ok(Self { pool, max_items })
        }

        pub async fn count(&self) -> Result<usize> {
            let row = sqlx::query("SELECT COUNT(*) as count FROM history")
                .fetch_one(&self.pool)
                .await?;
            Ok(row.get::<i64, _>("count") as usize)
        }

        pub async fn cleanup(&self) -> Result<()> {
            let count = self.count().await?;
            if count > self.max_items {
                let excess = count - self.max_items;
                sqlx::query(
                    "DELETE FROM history WHERE id IN (SELECT id FROM history ORDER BY last_visit ASC LIMIT ?)"
                )
                .bind(excess as i64)
                .execute(&self.pool)
                .await?;
            }
            Ok(())
        }
    }
}

mod bookmarks {
    use super::*;

    pub struct BookmarkManager {
        pool: SqlitePool,
        max_items: usize,
    }

    impl BookmarkManager {
        pub async fn new(max_items: usize, pool: SqlitePool) -> Result<Self> {
            sqlx::query(
                r#"
                CREATE TABLE IF NOT EXISTS bookmarks (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    url TEXT NOT NULL,
                    title TEXT NOT NULL,
                    folder TEXT DEFAULT 'default',
                    created_at TEXT NOT NULL
                )
                "#,
            )
            .execute(&pool)
            .await?;

            Ok(Self { pool, max_items })
        }

        pub async fn count(&self) -> Result<usize> {
            let row = sqlx::query("SELECT COUNT(*) as count FROM bookmarks")
                .fetch_one(&self.pool)
                .await?;
            Ok(row.get::<i64, _>("count") as usize)
        }

        pub async fn cleanup(&self) -> Result<()> {
            // Bookmarks don't need automatic cleanup
            Ok(())
        }
    }
}
