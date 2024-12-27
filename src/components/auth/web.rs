use reqwest::{Client, Method, Request, Url};

use crate::{common::Region, routes, HoyoError};

struct WebAuthClient<'a> {
    client: &'a Client,
}

impl<'a> WebAuthClient<'a> {
    fn _cn_web_login(&self, account: &str, password: &str) -> Result<Request, HoyoError>{
        let request = Request::new(Method::POST, Url::parse(routes::web_login_url(Region::Chinese))?);



        todo!()
    }
}
