use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Download {
    pub id: String,
    pub url: String,
    pub filename: String,
    pub path: PathBuf,
    pub total_bytes: Option<u64>,
    pub received_bytes: u64,
    pub status: DownloadStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DownloadStatus {
    Pending,
    InProgress,
    Paused,
    Completed,
    Failed,
}

impl Download {
    pub fn new(url: String, filename: String, path: PathBuf) -> Self {
        Download {
            id: uuid::Uuid::new_v4().to_string(),
            url,
            filename,
            path,
            total_bytes: None,
            received_bytes: 0,
            status: DownloadStatus::Pending,
        }
    }

    pub fn progress(&self) -> f64 {
        if let Some(total) = self.total_bytes {
            if total > 0 {
                return (self.received_bytes as f64 / total as f64) * 100.0;
            }
        }
        0.0
    }
}

