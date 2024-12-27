use crate::{common::Region, models::auth::cookie::{CNWebLoginResult, WebLoginResult}, HoyoError};

mod app;
mod game;
mod web;

/// Auth client
pub struct AuthClient {
    region: Region,
}

impl AuthClient {
    #[maybe_async::maybe_async]
    pub async fn login_with_password(
        &self,
        account: &str,
        password: &str,
    ) -> Result<WebLoginResult, HoyoError> {
        match self.region {
            Region::Chinese => return self.cn_login_with_password(),
            Region::Overseas => {}
        }

        todo!()
    }

    #[maybe_async::maybe_async]
    async fn cn_login_with_password(&self, account: &str, password: &str) -> Result<CNWebLoginResult, HoyoError>{
        
    }
}
