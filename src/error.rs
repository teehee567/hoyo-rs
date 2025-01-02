use reqwest::header::InvalidHeaderValue;
use thiserror::Error;

use crate::models::auth::geetest::SessionMMT;

/// HoyoApi Error
#[derive(Debug, Error)]
pub enum HoyoError {
    /// Could not parse language from str.
    #[error("Could not parse language")]
    LanguageParse,

    /// Unexpected Response.
    #[error("Unexpected Response: {0}")]
    UnexpectedResponse(String),

    /// Hoyolab Response Error.
    #[error(transparent)]
    Hoyolab(#[from] HoyolabError),

    /// Reqwest Errors.
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    /// Reqwest Header Error.
    #[error("Header transform error: {0}")]
    InvalidHeader(#[from] InvalidHeaderValue),

    /// Serde Error.
    #[error("Serde error: {0}")]
    SerdeError(#[from] serde_json::Error),
}

/// Errors from Hoyolab Response.
#[derive(Debug, Error)]
pub enum HoyolabError {
    /// Response data empty
    #[error("Empty data in response")]
    EmptyResponseData,

    /// Invalid Cookies.
    #[error("Invalid cookies.")]
    InvalidCookies,

    /// Invalid Lanuage from Hoyolab
    #[error("Invalid language.")]
    InvalidLanguage,

    /// Visited a page too frequently.
    #[error("Visits too frequently.")]
    VisitsTooFrequently,

    /// Malforemd request.
    #[error("Malformed request.")]
    MalformedRequest,

    /// No game account wiht cookie found.
    #[error("No game account associated with cookies.")]
    NoGameAccountWithCookies,

    /// Made too mahy request and got ratelimited.
    /// Cannot get data for more than 30 accounts per cookie per day.
    #[error("Too many requests.")]
    TooManyRequests,

    /// User's data is not public.
    #[error("Data not public.")]
    DataNotPublic,

    /// Cannot view data of other peoples notes.
    #[error("Cannot view real-time notes of other users.")]
    CannotViewRealTimeNotes,

    /// Invalid cookies.
    #[error("Cookies are valid but do not have a Hoyolab account bound to them.")]
    CookiesNotBound,

    /// Internal Database error.
    #[error("Internal database error.")]
    InternalDatabaseError,

    /// Hoyolab account has no game account bound to it.
    #[error("Account not found, uid may be invalid.")]
    AccountNotFound,

    /// Game account is incorrect.
    #[error("Incorrect game account.")]
    IncorrectGameAccount,

    /// Incorrect game password.
    #[error("Incorrect game password.")]
    IncorrectGamePassword,

    /// This action must be done in the app.
    #[error("This action must be done in the app.")]
    ActionMustBeInApp,

    /// Hoyolab captcha triggered.
    #[error("Geetest Triggered")]
    Captcha(SessionMMT),

    /// Redemption code invalid.
    #[error("Redemption code invalid.")]
    RedemptionInvalid,

    /// Redemption is on cooldown.
    #[error("Redemption is on cooldown.")]
    RedemptionCooldown,

    /// Redemption code has already been claimed.
    #[error("Redemption code already claimed.")]
    RedemptionClaimed,

    /// Redemption code has already expired.
    #[error("Redemption code has expired.")]
    RedemptionCodeExpired,

    /// REdemptin code is incorret formatted
    #[error("Redemption code is incorrectly formatted.")]
    RedemptionCodeMalformed,

    /// Redemption code is not activated
    #[error("Redemption code not activated.")]
    RedemptionCodeNotActivated,

    /// Adventure rank too low to claim code.
    #[error("Cannot claim codes for accounts with adventure rank lower than 10.")]
    RedemptionAdventureRankTooLow,

    /// No hoyolab account or password incorrect.
    #[error("Account login failed.")]
    AccountLoginFail,

    /// Account has been locked after incorrect psasword input,
    /// Happens after 3-5 times.
    #[error("Account has been locked after incorrect password")]
    AccountLocked,

    /// Account does not exist.
    #[error("Account does not exist.")]
    AccountDoesNotExist,

    /// Wrong OTP code.
    #[error("Wrong OTP.")]
    WrongOTP,

    /// Too many verification code requests for account.
    #[error("Verification code rate limited.")]
    VerificationCodeRateLimited,

    /// Too manh OTP message set for number,
    /// limit is around 40 messages/day/number.
    #[error("OTP rate limited.")]
    OTPRateLimited,

    /// OTP Request too frequent.
    #[error("Request too frequent.")]
    RequestTooFrequent,

    /// Reward already claimed.
    #[error("Reward already claimed.")]
    RewardAlreadyClaimed,

    /// Unknown error
    #[error("Unknown error code: {0}")]
    UnknownError(i32),
}

impl HoyolabError {
    /// Converts a error status code into HoyoError equivalent.
    pub(crate) fn from_code(code: i32) -> Self {
        match code {
            // Hoyolab Errors
            -100 => HoyolabError::InvalidCookies,
            -108 => HoyolabError::InvalidLanguage,
            -110 => HoyolabError::VisitsTooFrequently,

            // Game Record Errors
            10001 => HoyolabError::InvalidCookies,
            -10001 => HoyolabError::MalformedRequest,
            -10002 => HoyolabError::NoGameAccountWithCookies,

            // Database Game Record Errors
            10101 => HoyolabError::TooManyRequests,
            10102 => HoyolabError::DataNotPublic,
            10103 => HoyolabError::CookiesNotBound,
            10104 => HoyolabError::CannotViewRealTimeNotes,

            // Calculator Errors
            -500001 => HoyolabError::InvalidLanguage,
            -500004 => HoyolabError::VisitsTooFrequently,
            -502001 => HoyolabError::InvalidLanguage,
            -502002 => HoyolabError::InvalidLanguage,

            // Redemption Errors
            -1065 => HoyolabError::RedemptionInvalid,
            -1071 => HoyolabError::InvalidCookies,
            -1073 => HoyolabError::AccountNotFound,
            -2001 => HoyolabError::RedemptionCodeExpired,
            -2003 => HoyolabError::RedemptionCodeMalformed,
            -2004 => HoyolabError::RedemptionInvalid,
            -2014 => HoyolabError::RedemptionCodeNotActivated,
            -2016 => HoyolabError::RedemptionCooldown,
            -2017 | -2018 => HoyolabError::RedemptionClaimed,
            -2021 => HoyolabError::RedemptionAdventureRankTooLow,

            // Rewards Errors
            -5003 => HoyolabError::RewardAlreadyClaimed,

            // Chinese Errors
            1008 => HoyolabError::AccountNotFound,
            -1104 => HoyolabError::ActionMustBeInApp,

            // Account Errors
            -3208 => HoyolabError::AccountLoginFail,
            -3202 => HoyolabError::AccountLocked,
            -3203 => HoyolabError::AccountDoesNotExist,
            -3205 => HoyolabError::WrongOTP,
            -3206 => HoyolabError::VerificationCodeRateLimited,

            // Miyoushe Errors
            -119 => HoyolabError::OTPRateLimited,
            -3006 => HoyolabError::RequestTooFrequent,

            // Game Login Errors
            -216 => HoyolabError::IncorrectGameAccount,
            -202 => HoyolabError::IncorrectGamePassword,

            _ => HoyolabError::UnknownError(code),
        }
    }
}

pub(crate) fn check_captcha() {}
