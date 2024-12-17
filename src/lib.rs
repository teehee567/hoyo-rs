#![deny(missing_docs)]

//! # hoyo-rs
//! API wrapper for Genshin Impact & Honkai Star Rail.

mod components;
pub use components::*;

mod config;

mod client;
use client::Client;

/// Client to use Hoyolab API
#[derive(Clone)]
pub struct Hoyo {
    config: config::Config,
    client: Client,
}

impl Hoyo {
    
}
