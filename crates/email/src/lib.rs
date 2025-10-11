use anyhow::Result;
use async_imap::{Client as ImapClient, Session};
use async_smtp::Client as SmtpClient;
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, Row};
use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub mod providers;
pub mod cache;
pub mod auth;

use providers::{EmailProvider, ProviderType};
use cache::EmailCache;
use auth::{AuthToken, AuthFlow};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailConfig {
    pub provider: ProviderType,
    pub server: EmailServerConfig,
    pub auth: AuthConfig,
    pub cache_size_mb: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailServerConfig {
    pub imap_host: String,
    pub imap_port: u16,
    pub imap_security: SecurityType,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_security: SecurityType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityType {
    None,
    StartTls,
    Tls,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub username: Option<String>,
    pub oauth2: Option<OAuth2Config>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2Config {
    pub client_id: String,
    pub client_secret: String,
    pub auth_url: String,
    pub token_url: String,
    pub scope: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailHeader {
    pub uid: u32,
    pub subject: String,
    pub from: String,
    pub to: String,
    pub date: DateTime<Utc>,
    pub size: usize,
    pub flags: Vec<String>,
    pub folder: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailBody {
    pub uid: u32,
    pub content: String,
    pub content_type: String,
    pub attachments: Vec<Attachment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub filename: String,
    pub content_type: String,
    pub size: usize,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComposeEmail {
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub bcc: Vec<String>,
    pub subject: String,
    pub body: String,
    pub attachments: Vec<Attachment>,
}

pub struct EmailClient {
    config: EmailConfig,
    cache: EmailCache,
    providers: HashMap<ProviderType, Box<dyn EmailProvider + Send + Sync>>,
    auth_tokens: RwLock<HashMap<String, AuthToken>>,
    pool: SqlitePool,
}

impl EmailClient {
    pub async fn new(config: EmailConfig) -> Result<Self> {
        // Initialize database
        let pool = SqlitePool::connect("sqlite:email_cache.db").await?;
        
        // Run migrations
        sqlx::migrate!("./migrations").run(&pool).await?;

        // Initialize cache
        let cache = EmailCache::new(config.cache_size_mb, pool.clone()).await?;

        // Initialize providers
        let mut providers: HashMap<ProviderType, Box<dyn EmailProvider + Send + Sync>> = HashMap::new();
        
        match config.provider {
            ProviderType::Gmail => {
                providers.insert(ProviderType::Gmail, Box::new(providers::GmailProvider::new()));
            }
            ProviderType::Outlook => {
                providers.insert(ProviderType::Outlook, Box::new(providers::OutlookProvider::new()));
            }
            ProviderType::Yahoo => {
                providers.insert(ProviderType::Yahoo, Box::new(providers::YahooProvider::new()));
            }
            ProviderType::GenericImap => {
                providers.insert(ProviderType::GenericImap, Box::new(providers::GenericImapProvider::new()));
            }
        }

        Ok(Self {
            config,
            cache,
            providers,
            auth_tokens: RwLock::new(HashMap::new()),
            pool,
        })
    }

    pub async fn authenticate(&mut self, flow: AuthFlow) -> Result<()> {
        if let Some(provider) = self.providers.get(&self.config.provider) {
            let token = provider.authenticate(&flow).await?;
            let token_id = Uuid::new_v4().to_string();
            
            let mut tokens = self.auth_tokens.write().await;
            tokens.insert(token_id, token);
        }
        Ok(())
    }

    pub async fn sync_headers(&self, folder: &str) -> Result<Vec<EmailHeader>> {
        if let Some(provider) = self.providers.get(&self.config.provider) {
            let headers = provider.sync_headers(folder).await?;
            
            // Cache headers
            self.cache.store_headers(&headers).await?;
            
            Ok(headers)
        } else {
            Err(anyhow::anyhow!("Provider not found"))
        }
    }

    pub async fn fetch_body(&self, uid: u32) -> Result<EmailBody> {
        // Check cache first
        if let Some(body) = self.cache.get_body(uid).await? {
            return Ok(body);
        }

        // Fetch from provider
        if let Some(provider) = self.providers.get(&self.config.provider) {
            let body = provider.fetch_body(uid).await?;
            
            // Cache body
            self.cache.store_body(&body).await?;
            
            Ok(body)
        } else {
            Err(anyhow::anyhow!("Provider not found"))
        }
    }

    pub async fn send_email(&self, email: ComposeEmail) -> Result<()> {
        if let Some(provider) = self.providers.get(&self.config.provider) {
            provider.send_email(email).await?;
        } else {
            Err(anyhow::anyhow!("Provider not found"))
        }
        Ok(())
    }

    pub async fn search_emails(&self, query: &str) -> Result<Vec<EmailHeader>> {
        self.cache.search_headers(query).await
    }

    pub async fn get_memory_usage(&self) -> Result<usize> {
        self.cache.get_memory_usage().await
    }

    pub async fn cleanup_cache(&self) -> Result<()> {
        self.cache.cleanup().await
    }
}

// Memory budget enforcement
impl EmailClient {
    pub async fn enforce_memory_limits(&self) -> Result<()> {
        let current_usage = self.get_memory_usage().await?;
        let max_usage = self.config.cache_size_mb * 1024 * 1024;

        if current_usage > max_usage {
            // Remove oldest cached items
            self.cache.cleanup().await?;
        }

        Ok(())
    }
}
