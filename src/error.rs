use thiserror::Error;

#[derive(Debug, Error)]
pub enum HoyoError {
    #[error(transparent)]
    Hoyolab(#[from] HoyolabError),

    // Redemption Errors
    #[error(transparent)]
    Redemption(#[from] RedemptionError),

    // Account Errors
    #[error(transparent)]
    Account(#[from] AccountError),

    // Account Errors
    #[error(transparent)]
    Daily(#[from] DailyError),

    // External Errors
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Serde error: {0}")]
    SerdeError(#[from] serde_json::Error),

    // Unknown error
    #[error("Unknown error code: {0}")]
    UnknownError(i32),
}

#[derive(Debug, Error)]
pub enum HoyolabError {
    #[error("Invalid cookies.")]
    InvalidCookies,

    #[error("Invalid language.")]
    InvalidLanguage,

    #[error("Visits too frequently.")]
    VisitsTooFrequently,

    #[error("Malformed request.")]
    MalformedRequest,

    #[error("No game account associated with cookies.")]
    NoGameAccountWithCookies,

    #[error("Too many requests.")]
    TooManyRequests,

    #[error("Data not public.")]
    DataNotPublic,

    #[error("Cannot view real-time notes of other users.")]
    CannotViewRealTimeNotes,

    #[error("Cookies are valid but do not have a Hoyolab account bound to them.")]
    CookiesNotBound,

    #[error("Internal database error.")]
    InternalDatabaseError,

    #[error("Account not found.")]
    AccountNotFound,

    #[error("Incorrect game account.")]
    IncorrectGameAccount,

    #[error("Incorrect game password.")]
    IncorrectGamePassword,

    #[error("This action must be done in the app.")]
    ActionMustBeInApp,

    #[error("Geetest Triggered")]
    Captcha,
}

#[derive(Debug, Error)]
pub enum RedemptionError {
    #[error("Redemption invalid.")]
    RedemptionInvalid,

    #[error("Redemption exception: {0}")]
    RedemptionException(String),
    
    #[error("Redemption cooldown.")]
    RedemptionCooldown,

    #[error("Redemption claimed.")]
    RedemptionClaimed,

    #[error("Redemption code has expired.")]
    RedemptionCodeExpired,

    #[error("Redemption code is incorrectly formatted.")]
    RedemptionCodeMalformed,

    #[error("Redemption code not activated.")]
    RedemptionCodeNotActivated,

    #[error("Cannot claim codes for accounts with adventure rank lower than 10.")]
    RedemptionAdventureRankTooLow,
}


#[derive(Debug, Error)]
pub enum AccountError {
    #[error("Account login failed.")]
    AccountLoginFail,

    #[error("Account has locked.")]
    AccountHasLocked,

    #[error("Account does not exist.")]
    AccountDoesNotExist,

    #[error("Wrong OTP.")]
    WrongOTP,

    #[error("Verification code rate limited.")]
    VerificationCodeRateLimited,

    #[error("OTP rate limited.")]
    OTPRateLimited,

    #[error("Request too frequent.")]
    RequestTooFrequent,
}

#[derive(Debug, Error)]
pub enum DailyError {
    #[error("Already claimed.")]
    AlreadyClaimed,
}


impl HoyoError {
    pub fn from_code(code: i32) -> Self {
        match code {
            // Hoyolab Errors
            -100 => HoyoError::Hoyolab(HoyolabError::InvalidCookies),
            -108 => HoyoError::Hoyolab(HoyolabError::InvalidLanguage),
            -110 => HoyoError::Hoyolab(HoyolabError::VisitsTooFrequently),

            // Game Record Errors
            10001 => HoyoError::Hoyolab(HoyolabError::InvalidCookies),
            -10001 => HoyoError::Hoyolab(HoyolabError::MalformedRequest),
            -10002 => HoyoError::Hoyolab(HoyolabError::NoGameAccountWithCookies),

            // Database Game Record Errors
            10101 => HoyoError::Hoyolab(HoyolabError::TooManyRequests),
            10102 => HoyoError::Hoyolab(HoyolabError::DataNotPublic),
            10103 => HoyoError::Hoyolab(HoyolabError::CookiesNotBound),
            10104 => HoyoError::Hoyolab(HoyolabError::CannotViewRealTimeNotes),

            // Calculator Errors
            -500001 => HoyoError::Hoyolab(HoyolabError::InvalidLanguage),
            -500004 => HoyoError::Hoyolab(HoyolabError::VisitsTooFrequently),
            -502001 => HoyoError::Hoyolab(HoyolabError::InvalidLanguage),
            -502002 => HoyoError::Hoyolab(HoyolabError::InvalidLanguage),

            // Redemption Errors
            -1065 => HoyoError::Redemption(RedemptionError::RedemptionInvalid),
            -1071 => HoyoError::Hoyolab(HoyolabError::InvalidCookies),
            -1073 => HoyoError::Hoyolab(HoyolabError::AccountNotFound),
            -2001 => HoyoError::Redemption(RedemptionError::RedemptionCodeExpired),
            -2003 => HoyoError::Redemption(RedemptionError::RedemptionCodeMalformed),
            -2004 => HoyoError::Redemption(RedemptionError::RedemptionInvalid),
            -2014 => HoyoError::Redemption(RedemptionError::RedemptionCodeNotActivated),
            -2016 => HoyoError::Redemption(RedemptionError::RedemptionCooldown),
            -2017 | -2018 => HoyoError::Redemption(RedemptionError::RedemptionClaimed),
            -2021 => HoyoError::Redemption(RedemptionError::RedemptionAdventureRankTooLow),

            // Rewards Errors
            -5003 => HoyoError::Daily(DailyError::AlreadyClaimed),

            // Chinese Errors
            1008 => HoyoError::Hoyolab(HoyolabError::AccountNotFound),
            -1104 => HoyoError::Hoyolab(HoyolabError::ActionMustBeInApp),

            // Account Errors
            -3208 => HoyoError::Account(AccountError::AccountLoginFail),
            -3202 => HoyoError::Account(AccountError::AccountHasLocked),
            -3203 => HoyoError::Account(AccountError::AccountDoesNotExist),
            -3205 => HoyoError::Account(AccountError::WrongOTP),
            -3206 => HoyoError::Account(AccountError::VerificationCodeRateLimited),

            // Miyoushe Errors
            -119 => HoyoError::Account(AccountError::OTPRateLimited),
            -3006 => HoyoError::Account(AccountError::RequestTooFrequent),

            // Game Login Errors
            -216 => HoyoError::Hoyolab(HoyolabError::IncorrectGameAccount),
            -202 => HoyoError::Hoyolab(HoyolabError::IncorrectGamePassword),

            _ => HoyoError::UnknownError(code),
        }
    }
}

pub fn check_captcha() {

}
