use reqwest::{header::HeaderMap, Client, Method, Request, Url};

use crate::{utils::{common::{headermap, Region}, routes}, HoyoError};

struct WebAuthClient<'a> {
    client: &'a Client,
}

impl<'a> WebAuthClient<'a> {
    fn _cn_web_login(&self, account: &str, password: &str) -> Result<Request, HoyoError> {
        let mut request = Request::new(
            Method::POST,
            // expect because routes are in const as part of library
            Url::parse(routes::web_login_url(Region::Chinese)).expect("failed to parse url"),
        );

        Ok(request)
    }
}
