use serde::Deserialize;

/// Result from fetching `stoken` with `fetch_stoken_by_game_token`.
#[derive(Deserialize)]
pub struct STokenResult {
    pub aid: String,
    pub mid: String,
    pub token: String,
}

/// QR code login cookies.
#[derive(Deserialize)]
pub struct QRLoginResult {
    pub cookie_token_v2: String,
    pub account_mid_v2: String,
    pub account_id_v2: String,
    pub ltoken_v2: String,
    pub ltmid_v2: String,
    pub ltuid_v2: String,
}

/// Mobile App code login cookies.
#[derive(Deserialize)]
pub struct AppLoginResult {
    pub stoken: String,
    pub ltuid_v2: String,
    pub ltmid_v2: String,
    pub account_id_v2: String,
    pub account_mid_v2: String,
}

/// Hoyolab website login cookies.
#[derive(Deserialize)]
pub struct WebLoginResult {
    pub cookie_token_v2: String,
    pub account_mid_v2: String,
    pub account_id_v2: String,
    pub ltoken_v2: String,
    pub ltmid_v2: String,
    pub ltuid_v2: String,
}

pub type CNWebLoginResult = WebLoginResult;

/// Mobile login result, using phone number.
#[derive(Deserialize)]
pub struct MobileLoginResult {
    pub cookie_token_v2: String,
    pub account_mid_v2: String,
    pub account_id_v2: String,
    pub ltoken_v2: String,
    pub ltmid_v2: String,
}

/// Device grant result.
#[derive(Deserialize)]
pub struct DeviceGrantResult {
    pub game_token: String,
    pub login_ticket: Option<String>,
}

/// Gaem login result.
#[derive(Deserialize)]
pub struct GameLoginResult {
    pub combo_id: String,
    pub open_id: String,
    pub combo_token: String,
    pub heartbeat: bool,
    pub account_type: i32,
}
