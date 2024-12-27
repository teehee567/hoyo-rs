/// Result from fetching `stoken` with `fetch_stoken_by_game_token`.
pub struct STokenResult {
    aid: String,
    mid: String,
    token: String,
}

/// QR code login cookies.
pub struct QRLoginResult {
    cookie_token_v2: String,
    account_mid_v2: String,
    account_id_v2: String,
    ltoken_v2: String,
    ltmid_v2: String,
    ltuid_v2: String,
}

/// Mobile App code login cookies.
pub struct AppLoginResult {
    stoken: String,
    ltuid_v2: String,
    ltmid_v2: String,
    account_id_v2: String,
    account_mid_v2: String,
}

/// Hoyolab website login cookies.
pub struct WebLoginResult {
    cookie_token_v2: String,
    account_mid_v2: String,
    account_id_v2: String,
    ltoken_v2: String,
    ltmid_v2: String,
    ltuid_v2: String,
}

/// miyoushi webstie login cookies.
pub struct CNWebLoginResult {
    cookie_token_v2: String,
    account_mid_v2: String,
    account_id_v2: String,
    ltoken_v2: String,
    ltmid_v2: String,
    ltuid_v2: String,
}

/// Mobile login result, using phone number.
pub struct MobileLoginResult {
    cookie_token_v2: String,
    account_mid_v2: String,
    account_id_v2: String,
    ltoken_v2: String,
    ltmid_v2: String,
}

/// Device grant result.
pub struct DeviceGrantResult {
    game_token: String,
    login_ticket: Option<String>,
}

/// Gaem login result.
pub struct GameLoginResult {
    combo_id: String,
    open_id: String,
    combo_token: String,
    heartbeat: bool,
    account_type: i32,
}
