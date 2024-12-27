#![deny(missing_docs)]

//! # hoyo-rs
//! API wrapper for Genshin Impact & Honkai Star Rail.

mod components;
pub use components::*;
pub mod models;
pub use models::*;
pub mod error;
pub use error::*;

mod routes;

mod config;

mod client;
use client::Client;

mod common;

/// Client to use Hoyolab API
#[derive(Clone)]
pub struct Hoyo {
    config: config::Config,
    client: Client,
}

impl Hoyo {
    
}
