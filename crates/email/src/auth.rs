use anyhow::Result;
use oauth2::{ClientId, ClientSecret, AuthUrl, TokenUrl, Scope};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthFlow {
    DeviceCode {
        user_code: String,
        device_code: String,
        verification_url: String,
    },
    WebFlow {
        auth_url: String,
        state: String,
    },
    UsernamePassword {
        username: String,
        password: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub scope: Option<String>,
}

pub struct AuthManager {
    tokens: HashMap<String, AuthToken>,
    clients: HashMap<String, BasicClient>,
}

impl AuthManager {
    pub fn new() -> Self {
        Self {
            tokens: HashMap::new(),
            clients: HashMap::new(),
        }
    }

    pub fn register_gmail_client(&mut self, client_id: String, client_secret: String) -> Result<()> {
        let client = BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())?,
            Some(TokenUrl::new("https://oauth2.googleapis.com/token".to_string())?),
        );

        self.clients.insert("gmail".to_string(), client);
        Ok(())
    }

    pub fn register_outlook_client(&mut self, client_id: String, client_secret: String) -> Result<()> {
        let client = BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new("https://login.microsoftonline.com/common/oauth2/v2.0/authorize".to_string())?,
            Some(TokenUrl::new("https://login.microsoftonline.com/common/oauth2/v2.0/token".to_string())?),
        );

        self.clients.insert("outlook".to_string(), client);
        Ok(())
    }

    pub fn register_yahoo_client(&mut self, client_id: String, client_secret: String) -> Result<()> {
        let client = BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new("https://api.login.yahoo.com/oauth2/request_auth".to_string())?,
            Some(TokenUrl::new("https://api.login.yahoo.com/oauth2/get_token".to_string())?),
        );

        self.clients.insert("yahoo".to_string(), client);
        Ok(())
    }

    pub async fn start_device_flow(&self, provider: &str) -> Result<AuthFlow> {
        if let Some(client) = self.clients.get(provider) {
            // Start device code flow
            let device_code_request = client
                .exchange_device_code()?
                .add_scope(Scope::new("https://www.googleapis.com/auth/gmail.readonly".to_string()))
                .add_scope(Scope::new("https://www.googleapis.com/auth/gmail.send".to_string()));

            let device_response = device_code_request.request_async(async_http_client).await?;

            Ok(AuthFlow::DeviceCode {
                user_code: device_response.user_code().secret().clone(),
                device_code: device_response.device_code().secret().clone(),
                verification_url: device_response.verification_uri().to_string(),
            })
        } else {
            Err(anyhow::anyhow!("Provider not registered: {}", provider))
        }
    }

    pub async fn complete_device_flow(&self, provider: &str, device_code: &str) -> Result<AuthToken> {
        if let Some(client) = self.clients.get(provider) {
            let token_request = client
                .exchange_device_code(&oauth2::DeviceCode::new(device_code.to_string()));

            let token_response = token_request.request_async(async_http_client).await?;

            Ok(AuthToken {
                access_token: token_response.access_token().secret().clone(),
                refresh_token: token_response.refresh_token().map(|t| t.secret().clone()),
                expires_at: token_response.expires_in().map(|duration| {
                    chrono::Utc::now() + chrono::Duration::seconds(duration.as_secs() as i64)
                }),
                scope: token_response.scopes().map(|scopes| {
                    scopes.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(" ")
                }),
            })
        } else {
            Err(anyhow::anyhow!("Provider not registered: {}", provider))
        }
    }

    pub fn store_token(&mut self, provider: String, token: AuthToken) {
        self.tokens.insert(provider, token);
    }

    pub fn get_token(&self, provider: &str) -> Option<&AuthToken> {
        self.tokens.get(provider)
    }

    pub async fn refresh_token(&self, provider: &str) -> Result<AuthToken> {
        if let (Some(client), Some(token)) = (self.clients.get(provider), self.tokens.get(provider)) {
            if let Some(refresh_token) = &token.refresh_token {
                let refresh_request = client
                    .exchange_refresh_token(&oauth2::RefreshToken::new(refresh_token.clone()));

                let token_response = refresh_request.request_async(async_http_client).await?;

                Ok(AuthToken {
                    access_token: token_response.access_token().secret().clone(),
                    refresh_token: token_response.refresh_token().map(|t| t.secret().clone()),
                    expires_at: token_response.expires_in().map(|duration| {
                        chrono::Utc::now() + chrono::Duration::seconds(duration.as_secs() as i64)
                    }),
                    scope: token_response.scopes().map(|scopes| {
                        scopes.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(" ")
                    }),
                })
            } else {
                Err(anyhow::anyhow!("No refresh token available"))
            }
        } else {
            Err(anyhow::anyhow!("Provider or token not found"))
        }
    }

    pub fn is_token_expired(&self, provider: &str) -> bool {
        if let Some(token) = self.tokens.get(provider) {
            if let Some(expires_at) = token.expires_at {
                return chrono::Utc::now() >= expires_at;
            }
        }
        false
    }
}
