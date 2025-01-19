use std::{collections::HashMap, sync::Arc};

use reqwest::Client;
use reqwest_cookie_store::CookieStoreMutex;
use serde_json::json;

use crate::{
    auth::{
        cookie::AppLoginResult,
        geetest::{SessionMMT, SessionMMTResult},
        qrcode::{QRCodeCreationResult, QRCodeStatus},
        verification::ActionTicket,
    },
    common::Base,
    utils::{
        auth_constants::{
            APP_LOGIN_HEADERS, EMAIL_SEND_HEADERS, EMAIL_VERIFY_HEADERS, QRCODE_HEADERS,
        },
        constants::DsSalt,
        ds::generate_dynamic_secret,
        routes,
    },
    HoyoError, HoyolabError,
};

pub(super) struct AppAuthClient<'a> {
    client: &'a Client,
    store: Arc<CookieStoreMutex>,
}
impl<'a> AppAuthClient<'a> {
    #[inline]
    pub(super) fn new(client: &'a Client, store: Arc<CookieStoreMutex>) -> Self {
        Self { client, store }
    }

    #[inline]
    #[maybe_async::maybe_async]
    pub(super) async fn _app_login(
        &self,
        enc_account: &str,
        enc_password: &str,
        mmt_result: Option<SessionMMTResult>,
        ticket: Option<ActionTicket>,
    ) -> Result<AppLoginResult, HoyoError> {
        let mut builder = self
            .client
            .post(routes::APP_LOGIN_URL)
            .headers(APP_LOGIN_HEADERS.clone())
            .header("ds", generate_dynamic_secret(DsSalt::AppLogin));

        if let Some(mmt) = mmt_result {
            builder = builder.header("x-rpc-aigis", mmt.get_aigis_header()?);
        }

        if let Some(ticket) = ticket {
            builder = builder.header("x-rpc-verify", ticket.to_rpc_verify_header());
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

        if data.retcode == -3101 {
            let aigis_header = response_headers
                .get("x-rpc-aigis")
                .expect("Aigis header not found in response.")
                .to_str()
                .expect("Could not parse Aigis header");

            let session_mmt: SessionMMT = serde_json::from_str(aigis_header)?;
            return Err(HoyolabError::Captcha(session_mmt))?;
        }

        if data.retcode == -3239 {
            let aigis_header = response_headers
                .get("x-rpc-verify")
                .expect("Email Verify header not found in response.")
                .to_str()
                .expect("Could not parse Email Verify header");

            let action_ticket: ActionTicket = serde_json::from_str(aigis_header)?;
            return Err(HoyolabError::EmailVerify(action_ticket))?;
        }

        if data.data.is_none() {
            return Err(HoyolabError::from_code(data.retcode))?;
        }

        Ok(serde_json::from_value(json!(cookies))?)
    }

    #[inline]
    #[maybe_async::maybe_async]
    pub(super) async fn _send_verification_email(
        &self,
        ticket: ActionTicket,
        mmt_result: Option<SessionMMTResult>,
    ) -> Result<(), HoyoError> {
        let mut builder = self
            .client
            .post(routes::SEND_VERIFICATION_CODE_URL)
            .headers(EMAIL_SEND_HEADERS.clone());

        if let Some(mmt) = mmt_result {
            builder = builder.header("x-rpc-aigis", mmt.get_aigis_header()?);
        }

        let response = builder
            .body(format!(
                r#"{{"action_type":"verify_for_component","action_ticket":"{}"}}"#,
                ticket.ticket
            ))
            .send()
            .await?;

        let response_headers = response.headers().clone();
        let data = response.json::<Base>().await?;

        if data.retcode == -3101 {
            let aigis_header = response_headers
                .get("x-rpc-aigis")
                .expect("Aigis header not found in response.")
                .to_str()
                .expect("Could not parse Aigis header");

            let session_mmt: SessionMMT = serde_json::from_str(aigis_header)?;
            return Err(HoyolabError::Captcha(session_mmt))?;
        }

        if data.retcode != 0 {
            return Err(HoyolabError::from_code(data.retcode))?;
        }
        Ok(())
    }

    #[inline]
    #[maybe_async::maybe_async]
    pub(super) async fn _verify_email(
        &self,
        code: &str,
        ticket: ActionTicket,
    ) -> Result<(), HoyoError> {
        let mut builder = self
            .client
            .post(routes::VERIFY_EMAIL_URL)
            .headers(EMAIL_VERIFY_HEADERS.clone());

        let response = builder
            .body(format!(
                r#"{{"action_type":"verify_for_component","action_ticket":"{}","email_captcha":"{}","verify_method":2}}"#,
                ticket.ticket, code
            ))
            .send()
            .await?;

        let data = response.json::<Base>().await?;
        if data.retcode != 0 {
            return Err(HoyolabError::from_code(data.retcode))?;
        }
        Ok(())
    }

    #[inline]
    #[maybe_async::maybe_async]
    pub(super) async fn _create_qrcode(&self) -> Result<QRCodeCreationResult, HoyoError> {
        let mut builder = self
            .client
            .post(routes::CREATE_QRCODE_URL)
            .headers(QRCODE_HEADERS.clone());

        let response = builder.send().await?;

        let data = response.json::<Base>().await?;

        if data.data.is_none() {
            return Err(HoyolabError::from_code(data.retcode))?;
        }

        if let Some(data) = data.data {
            Ok(serde_json::from_value(data)?)
        } else {
            Err(HoyolabError::from_code(data.retcode))?
        }
    }

    #[inline]
    #[maybe_async::maybe_async]
    pub(super) async fn _check_qrcode(&self, ticket: &str) -> Result<QRCodeStatus, HoyoError> {
        let mut builder = self
            .client
            .post(routes::CREATE_QRCODE_URL)
            .headers(QRCODE_HEADERS.clone());

        let response = builder
            .body(format!(r#"{{"ticket":{}}}"#, ticket))
            .send()
            .await?;

        let data = response.json::<Base>().await?;

        if data.data.is_none() {
            return Err(HoyolabError::from_code(data.retcode))?;
        }

        if let Some(mut data) = data.data {
            Ok(serde_json::from_value(
                data.get_mut("status")
                    .ok_or_else(|| HoyolabError::CouldNotGetStatus)?
                    .take(),
            )?)
        } else {
            Err(HoyolabError::from_code(data.retcode))?
        }
    }
}
