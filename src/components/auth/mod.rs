use std::{collections::HashMap, sync::Arc};

use reqwest_cookie_store::CookieStoreMutex;
use web::WebAuthClient;
use zeroize::Zeroizing;

use crate::{
    client::Client,
    common::Base,
    models::auth::{
        cookie::{CNWebLoginResult, WebLoginResult},
        geetest::{SessionMMT, SessionMMTResult},
    },
    utils::{auth_constants::hoyo_encrypt, common::Region, routes},
    HoyoError, HoyolabError,
};

use super::captcha::{CaptchaSolver, DefaultSolver};

mod app;
mod game;
mod web;

/// Auth client
pub struct AuthClient {
    client: Client,
    region: Region,
    cookie_store: Arc<CookieStoreMutex>,
}

impl AuthClient {
    pub(crate) fn new(client: Client, region: Region, cookie_store: Arc<CookieStoreMutex>) -> Self {
        Self {
            client,
            region,
            cookie_store,
        }
    }

    ///Login with a password via web endpoint
    ///This enpoint is mainly for convenience.
    ///
    ///Endpoint is chosen based on client region.
    ///
    ///Note that this will start a webserver if captcha is
    ///triggered and `geetest_solver` is not passed.
    #[maybe_async::maybe_async]
    pub async fn login_with_password(
        &self,
        account: &str,
        password: &str,
        solver: Option<impl CaptchaSolver>,
    ) -> Result<WebLoginResult, HoyoError> {
        let enc_account = hoyo_encrypt(account, self.region);
        let enc_password = hoyo_encrypt(password, self.region);

        match self.region {
            Region::Overseas => {
                self.os_login_with_password(&enc_account, &enc_password, 6, solver)
                    .await
            }
            Region::Chinese => {
                self.cn_login_with_password(&enc_account, &enc_password, solver)
                    .await
            }
        }
    }

    ///Login with a password via web endpoint
    ///
    ///Note that this will start a webserver if captcha is
    ///triggered and `geetest_solver` is not passed.
    #[maybe_async::maybe_async]
    async fn os_login_with_password(
        &self,
        enc_account: &str,
        enc_password: &str,
        token_type: i32,
        solver: Option<impl CaptchaSolver>,
    ) -> Result<WebLoginResult, HoyoError> {
        let client = WebAuthClient::new(&self.client, self.cookie_store.clone());

        match client
            ._os_web_login(enc_account, enc_password, None, token_type)
            .await
        {
            Err(HoyoError::Hoyolab(HoyolabError::Captcha(mmt))) => {
                let mmt_result = if let Some(solver) = solver {
                    solver.solve(mmt)
                } else {
                    DefaultSolver::new().solve(mmt)
                };
                client
                    ._os_web_login(enc_account, enc_password, Some(mmt_result), token_type)
                    .await
            }
            other => other,
        }
    }

    ///Login with a password via Miyoushi loginByPassword endpoint.
    ///
    ///Note that this will start a webserver if captcha is
    ///triggered and `geetest_solver` is not passed.
    #[maybe_async::maybe_async]
    async fn cn_login_with_password(
        &self,
        enc_account: &str,
        enc_password: &str,
        solver: Option<impl CaptchaSolver>,
    ) -> Result<CNWebLoginResult, HoyoError> {
        let client = WebAuthClient::new(&self.client, self.cookie_store.clone());

        match client._cn_web_login(enc_account, enc_password, None).await {
            Err(HoyoError::Hoyolab(HoyolabError::Captcha(mmt))) => {
                let mmt_result = if let Some(solver) = solver {
                    solver.solve(mmt)
                } else {
                    DefaultSolver::new().solve(mmt)
                };
                client
                    ._cn_web_login(enc_account, enc_password, Some(mmt_result))
                    .await
            }
            other => other,
        }
    }

    /// Check if a mobile number is valid (it's registered on Miyoushe).
    ///
    /// Returns Some(()) if mobile number is valid.
    #[maybe_async::maybe_async]
    pub async fn check_mobile_number_validity(&self, mobile: u64) -> Result<bool, HoyoError> {
        let response = self
            .client
            .get(routes::CHECK_MOBILE_VALIDITY_URL)
            .query(&("mobile", mobile.to_string()))
            .send()
            .await?;

        let data = response.json::<Base>().await?;
        
        if let Some(data) = data.data {
            let status = data.get("status").unwrap_or_else(|| {
                return Err(HoyoError::UnexpectedResponse("no status found".to_string()))?;
            });
            let is_registable = data.get("is_registable").unwrap_or_else(|| {
                return Err(HoyoError::UnexpectedResponse("no is_registable found".to_string()))?;
            });
            Ok(status != is_registable)
            
        } else {
            Err(HoyolabError::from_code(data.retcode))?
        }
    }

    /// Login with mobile number, returns cookies..
    ///
    /// Only works for Chinese region (Miyoushe) users, do not include
    /// area code (+86) in hte mobile number.
    ///
    /// Steps:
    /// 1. Sends OTP to the provided mobile number.
    /// 2. If captcha is triggered, prompts the user to solve it.
    /// 3. Lets user enter the OTP.
    /// 4. Logs in with the OTP.
    /// 5. Returns Cookies.
    ///
    #[maybe_async::maybe_async]
    pub async fn login_with_mobile_number(&self, mobile: u64) {
        let client = WebAuthClient::new(&self.client, self.cookie_store.clone());
        let enc_mobile = hoyo_encrypt(&mobile.to_string(), Region::Chinese);
        if let Err(HoyoError::Hoyolab(HoyolabError::Captcha(mmt))) = client._send_mobile_otp(&enc_mobile, None).await {

        }
        todo!()
    }

    /// Login with a password via HoYoLab app endpoint.
    ///
    /// Note that this will start a webserver if either of the
    /// following happens:
    ///
    /// 1. Captcha is triggered and `geetest_solver` is not passed.
    /// 2. Email verification is triggered (can happen if you first
    ///    login wiht a new device)
    ///
    #[maybe_async::maybe_async]
    pub async fn login_with_app_password(&self) {
        todo!()
    }

    /// Login with a QR code, only available for Chinese, Miyoushe users.
    #[maybe_async::maybe_async]
    pub async fn login_with_qrcode(&self) {
        todo!()
    }

    /// Creates a geetest challenge.
    #[maybe_async::maybe_async]
    pub async fn create_mmt(&self) {
        todo!()
    }

    /// Verifys a geetest challenge.
    #[maybe_async::maybe_async]
    pub async fn verify_mmt(&self) {
        todo!()
    }

    /// Performs a login to the game endpoint.
    #[maybe_async::maybe_async]
    pub async fn os_game_login(&self) {
        todo!()
    }

    /// Generate an authentic device fingerptint.
    #[maybe_async::maybe_async]
    pub async fn generate_fingerprint(&self) {
        todo!()
    }

    fn _gen_random_fingerprint(&self) -> String {
        todo!()
    }

    fn _gen_ext_fields(&self) -> String {
        todo!()
    }
}
