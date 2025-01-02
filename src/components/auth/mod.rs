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
    utils::{auth_constants::hoyo_encrypt, common::Region},
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
            Region::Overseas => self.os_login_with_password(&enc_account, &enc_password, 6, solver).await,
            Region::Chinese => self.cn_login_with_password(&enc_account, &enc_password, solver).await,
        }
    }

    #[maybe_async::maybe_async]
    async fn os_login_with_password(
        &self,
        enc_account: &str,
        enc_password: &str,
        token_type: i32,
        solver: Option<impl CaptchaSolver>,
    ) -> Result<WebLoginResult, HoyoError> {
        let client = WebAuthClient::new(&self.client, self.cookie_store.clone());

        match client._os_web_login(enc_account, enc_password, None, token_type).await {
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
            other => other
        }

    }

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
            other => other
        }
    }

    #[maybe_async::maybe_async]
    pub async fn check_mobile_number_validity(&self) {
        todo!()
    }

    #[maybe_async::maybe_async]
    pub async fn login_with_mobile_number(&self) {
        todo!()
    }

    #[maybe_async::maybe_async]
    pub async fn login_with_app_password(&self) {
        todo!()
    }

    #[maybe_async::maybe_async]
    pub async fn login_with_qrcode(&self) {
        todo!()
    }

    #[maybe_async::maybe_async]
    pub async fn create_mmt(&self) {
        todo!()
    }

    #[maybe_async::maybe_async]
    pub async fn verify_mmt(&self) {
        todo!()
    }

    #[maybe_async::maybe_async]
    pub async fn os_game_login(&self) {
        todo!()
    }

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
