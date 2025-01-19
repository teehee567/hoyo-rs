use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};

use crate::HoyoError;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum QRCodeStatus {
    Created,
    Scanned,
    Confirmed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QRCodeCreationResult {
    pub ticket: String,
    pub url: String,
}
