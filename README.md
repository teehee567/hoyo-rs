# hoyo-rs

A Rust library for interacting with the Hoyo-lab API for Genshin Impact, Honkai Impact 3rd, and other Hoyoverse games.

[![Crates.io](https://img.shields.io/crates/v/hoyo-rs)](https://crates.io/crates/hoyo-rs)
[![Documentation](https://docs.rs/hoyo-rs/badge.svg)](https://docs.rs/hoyo-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- **Fully Type-Safe**: Take advantage of Rust's type system for API responses
- **Async by Default**: Built on `tokio` and `reqwest` for efficient async operations
- **Error Handling**: Comprehensive error types with `thiserror`
- **Optional Blocking Support**: Use the blocking API with the `blocking` feature
- **TLS Flexibility**: Choose between `native-tls` or `rustls` backends

## Installation

Install through cargo:

```zsh
cargo add hoyo-rs
```

## Usage

```rust
use hoyo_rs::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a client with cookies
    let cookies = [
        ("ltuid", "119480035"),
        ("ltoken", "cnF7TiZqHAAvYqgCBoSPx5EjwezOh1ZHoqSHf7dT"),
    ];
    
    let client = Client::new().cookies(cookies);
    
    // Fetch Genshin Impact user data
    let user_data = client.get_genshin_user(710785423).await?;
    println!("User has a total of {} characters", user_data.stats.characters);
    
    Ok(())
}
```

### Blocking Usage

Enable the `blocking` feature and use the synchronous API:

```toml
[dependencies]
hoyo-rs = { version = "0.0.1", features = ["blocking"] }
```

```rust
use hoyo_rs::Client;

fn main() -> anyhow::Result<()> {
    let cookies = [
        ("ltuid", "119480035"),
        ("ltoken", "cnF7TiZqHAAvYqgCBoSPx5EjwezOh1ZHoqSHf7dT"),
    ];
    
    let client = Client::new().cookies(cookies);
    
    let user_data = client.get_genshin_user(710785423)?;
    println!("User has a total of {} characters", user_data.stats.characters);
    
    Ok(())
}
```

## Features

- **default**: Includes the `native-tls` feature
- **native-tls**: Uses the native TLS backend for HTTPS requests
- **rustls**: Uses the pure-Rust rustls backend for HTTPS requests
- **blocking**: Enables synchronous API methods

## Security Note

This library handles authentication tokens and cookies. Never share your tokens or expose them in client-side code.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgements

This library is inspired by [genshin.py](https://github.com/thesadru/genshin.py), a Python library for the Hoyoverse API.
