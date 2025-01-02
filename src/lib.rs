#![deny(missing_docs)]

//! # hoyo-rs
//! API wrapper for Genshin Impact & Honkai Star Rail.

/// Components for model interaction.
pub mod components;
use std::sync::Arc;


/// Response models.
pub mod models;
pub use models::*;

/// Error definition;
pub mod error;
pub use error::*;

mod utils;

mod config;

mod client;
use client::Client;
use components::auth::AuthClient;
use config::Config;
use reqwest_cookie_store::{CookieStore, CookieStoreMutex};
use utils::common::Region;

/// Client to use Hoyolab API
#[derive(Clone)]
pub struct Hoyo {
    config: config::Config,
    client: Client,
    cookie_store: Arc<CookieStoreMutex>,
}

impl Hoyo {
    /// Creates new HoyoApi instance.
    pub fn new() -> Self {
        Self::from_config(Config::default())
    }

    /// Creates new HoyoApi instance from config.
    pub fn from_config(config: Config) -> Self {
        let cookie_store = Arc::new(CookieStoreMutex::new(CookieStore::new(None)));
        let client = Client::builder()
            .cookie_provider(cookie_store.clone())
            .build()
            .expect("failed to build Request client");
        Self { config, client, cookie_store}
    }

    /// Creates an authentication client
    pub fn authclient(&self, region: Region) -> AuthClient {
        AuthClient::new(self.client.clone(), region, self.cookie_store.clone())
    }
}
