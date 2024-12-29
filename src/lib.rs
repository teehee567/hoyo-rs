#![deny(missing_docs)]

//! # hoyo-rs
//! API wrapper for Genshin Impact & Honkai Star Rail.

/// Components for model interaction.
mod components;
pub use components::*;

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

/// Client to use Hoyolab API
#[derive(Clone)]
pub struct Hoyo {
    config: config::Config,
    client: Client,
}

impl Hoyo {
    /// Creates new HoyoApi instance.
    pub fn new() {

    }


    /// Creates new HoyoApi instance from config.
    pub fn from_config() {

    }
    
}
