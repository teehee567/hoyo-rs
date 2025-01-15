use std::{collections::HashMap, sync::Arc};

use reqwest::Url;
use reqwest_cookie_store::{CookieStore, CookieStoreMutex, RawCookie};
use serde_json::json;

use crate::{
    auth::{cookie::MobileLoginResult, geetest::SessionMMTv4},
    client::Client,
    common::Base,
    models::auth::{
        cookie::{CNWebLoginResult, WebLoginResult},
        geetest::{SessionMMT, SessionMMTResult},
    },
    utils::{
        auth_constants::{hoyo_encrypt, CN_LOGIN_HEADERS, WEB_LOGIN_HEADERS},
        common::{headermap, Region},
        constants::DsSalt,
        ds::generate_dynamic_secret,
        routes,
    },
    HoyoError, HoyolabError,
};

pub(super) struct WebAuthClient<'a> {
    client: &'a Client,
    store: Arc<CookieStoreMutex>,
}

impl<'a> WebAuthClient<'a> {
    #[inline]
    pub(super) fn new(client: &'a Client, store: Arc<CookieStoreMutex>) -> Self {
        Self { client, store }
    }

    #[inline]
    #[maybe_async::maybe_async]
    pub(super) async fn _os_web_login(
        &self,
        enc_account: &str,
        enc_password: &str,
        mmt_result: Option<SessionMMTResult>,
        token_type: i32,
    ) -> Result<WebLoginResult, HoyoError> {
        let url = routes::web_login_url(Region::Overseas);
        let mut builder = self.client.post(url).headers(WEB_LOGIN_HEADERS.clone());
        

        if let Some(mmt) = mmt_result {
            builder = builder.header("x-rpc-aigis", mmt.get_aigis_header()?);
        }

        let response = builder
            .body(format!(
                r#"{{"account":"{}","password":"{}","token_type":"{}"}}"#,
                enc_account, enc_password, token_type
            ))
            .send()
            .await?;

        let response_headers = response.headers().clone();
        let cookies: HashMap<String, String> = response
            .cookies()
            .map(|c| (c.name().to_string(), c.value().to_string()))
            .collect();

        let data = response.json::<Base>().await?;

        if data.retcode == -3101 {
            // Captcha triggered
            let aigis_header = response_headers
                .get("x-rpc-aigis")
                .expect("Aigis header not found in response.")
                .to_str()
                .expect("Could not parse Aigis header");

            let session_mmt: SessionMMT = serde_json::from_str(aigis_header)?;
            return Err(HoyolabError::Captcha(session_mmt))?;
        }

        match data.data {
            Some(data) => {
                if let Some(token) = data.get("stoken") {
                    let token = token.to_string();
                    let cookie = RawCookie::new("stoken", token);
                    let mut store = self.store.lock().expect("Could not get Cookie store lock");
                    store.insert_raw(
                        &cookie,
                        &Url::parse(url).expect("Failed to parse cookie url"),
                    );
                }
            }
            None => return Err(HoyolabError::from_code(data.retcode))?,
        }

        Ok(serde_json::from_value(json!(cookies))?)
    }

    #[inline]
    #[maybe_async::maybe_async]
    pub(super) async fn _cn_web_login(
        &self,
        enc_account: &str,
        enc_password: &str,
        mmt_result: Option<SessionMMTResult>,
    ) -> Result<CNWebLoginResult, HoyoError> {
        let mut builder = self
            .client
            .post(routes::web_login_url(Region::Chinese))
            .headers(CN_LOGIN_HEADERS.clone())
            .header("ds", generate_dynamic_secret(DsSalt::Chinese));

        if let Some(mmt) = mmt_result {
            builder = builder.header("x-rpc-aigis", mmt.get_aigis_header()?);
        }

        let response = builder
            .body(format!(
                r#"{{"account":"{}","password":"{}"}}"#,
                enc_account, enc_password
            ))
            .send()
            .await?;

        let response_headers = response.headers().clone();
        let cookies: HashMap<String, String> = response
            .cookies()
            .map(|c| (c.name().to_string(), c.value().to_string()))
            .collect();

        let data = response.json::<Base>().await?;

        if data.retcode == -3102 {
            let aigis_header = response_headers
                .get("x-rpc-aigis")
                .expect("Aigis header not found in response.")
                .to_str()
                .expect("Could not parse Aigis header");

            let session_mmt: SessionMMT = serde_json::from_str(aigis_header)?;
            return Err(HoyolabError::Captcha(session_mmt))?;
        }

        if data.data.is_none() {
            return Err(HoyolabError::from_code(data.retcode))?;
        }

        Ok(serde_json::from_value(json!(cookies))?)
    }

    #[inline]
    #[maybe_async::maybe_async]
    pub(super) async fn _send_mobile_otp(
        &self,
        enc_mobile: &str,
        mmt_result: Option<SessionMMTResult>,
    ) -> Result<(), HoyoError> {
        let mut builder = self
            .client
            .post(routes::MOBILE_OTP_URL)
            .headers(CN_LOGIN_HEADERS.clone())
            .header("ds", generate_dynamic_secret(DsSalt::CnSignin));

        if let Some(mmt) = mmt_result {
            builder = builder.header("x-rpc-aigis", mmt.get_aigis_header()?);
        }

        let area_code = hoyo_encrypt("+86", Region::Chinese);

        let response = builder
            .body(format!(
                r#"{{"mobile":"{}","area_code":"{}"}}"#,
                enc_mobile, area_code
            ))
            .send()
            .await?;

        let response_headers = response.headers().clone();
        let data = response.json::<Base>().await?;

        if data.retcode == -3101 {
            // Captcha triggered
            let aigis_header = response_headers
                .get("x-rpc-aigis")
                .expect("Aigis header not found in response.")
                .to_str()
                .expect("Could not parse Aigis header");

            let session_mmt: SessionMMT = serde_json::from_str(aigis_header)?;
            return Err(HoyolabError::Captcha(session_mmt))?;
        }

        if data.data.is_none() {
            return Err(HoyolabError::from_code(data.retcode))?;
        }

        Ok(())
    }

    #[inline]
    #[maybe_async::maybe_async]
    pub(super) async fn _login_with_mobile_otp(
        &self,
        enc_mobile: &str,
        otp: &str,
    ) -> Result<MobileLoginResult, HoyoError> {
        let builder = self
            .client
            .post(routes::MOBILE_LOGIN_URL)
            .headers(CN_LOGIN_HEADERS.clone())
            .header("ds", generate_dynamic_secret(DsSalt::CnSignin));

        let area_code = hoyo_encrypt("+86", Region::Chinese);

        let response = builder
            .body(format!(
                r#"{{"mobile":"{}","area_code":"{}","captcha":"{}"}}"#,
                enc_mobile, area_code, otp
            ))
            .send()
            .await?;

        let cookies: HashMap<String, String> = response
            .cookies()
            .map(|c| (c.name().to_string(), c.value().to_string()))
            .collect();

        let data = response.json::<Base>().await?;

        if data.data.is_none() {
            return Err(HoyolabError::from_code(data.retcode))?;
        }

        Ok(serde_json::from_value(json!(cookies))?)
    }
}
