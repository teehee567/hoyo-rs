use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};

use crate::HoyoError;

#[derive(Debug, Serialize, Deserialize, Clone)]
/// Geetest verification data.
pub struct MMT {
    pub new_captcha: i32,
    pub success: i32,
    pub challenge: String,
    pub gt: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionMMT {
    pub new_captcha: i32,
    pub success: i32,
    pub challenge: String,
    pub gt: String,
    pub session_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RiskyCheckMMT {
    pub new_captcha: i32,
    pub success: i32,
    pub challenge: String,
    pub gt: String,
    pub check_id: String,
}

/// ------------------------------------
/// Geetest V4 (MMT) models
/// ------------------------------------

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MMTv4 {
    pub new_captcha: i32,
    pub success: i32,
    /// The Python code treats `captcha_id` as alias for `gt`.
    #[serde(alias = "gt")]
    pub captcha_id: String,
    pub risk_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionMMTv4 {
    /// Repeats the same fields as `MMTv4` plus `session_id`.
    pub new_captcha: i32,
    pub success: i32,
    #[serde(alias = "gt")]
    pub captcha_id: String,
    pub risk_type: String,
    pub session_id: String,
}

/// ------------------------------------
/// Result models
/// ------------------------------------

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MMTResult {
    /// In Python, it inherits from BaseMMTResult, but we keep it simple here.
    pub geetest_challenge: String,
    pub geetest_validate: String,
    pub geetest_seccode: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionMMTResult {
    pub geetest_challenge: String,
    pub geetest_validate: String,
    pub geetest_seccode: String,
    #[serde(skip_serializing)]
    pub session_id: String,
}

impl SessionMMTResult {
    pub(crate) fn get_aigis_header(&self) -> Result<String, HoyoError> {
        let json = serde_json::to_string(self)?;
        let encoded = general_purpose::STANDARD.encode(json);
        Ok(format!("{};{}", self.session_id, encoded))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MMTv4Result {
    pub captcha_id: String,
    pub lot_number: String,
    pub pass_token: String,
    pub gen_time: String,
    pub captcha_output: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionMMTv4Result {
    pub captcha_id: String,
    pub lot_number: String,
    pub pass_token: String,
    pub gen_time: String,
    pub captcha_output: String,
    pub session_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RiskyCheckMMTResult {
    pub geetest_challenge: String,
    pub geetest_validate: String,
    pub geetest_seccode: String,
    pub check_id: String,
}

/// ------------------------------------
/// Risky Check models
/// ------------------------------------

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RiskyCheckAction {
    ACTION_NONE,
    ACTION_GEETEST,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RiskyCheckResult {
    pub id: String,
    pub action: RiskyCheckAction,
    /// In Python, `mmt: typing.Optional[MMT] = Field(alias="geetest")`
    #[serde(alias = "geetest")]
    pub mmt: Option<MMT>,
}
