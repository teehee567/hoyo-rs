use std::collections::HashMap;

use chrono::Utc;
use rand::{
    distributions::{Alphanumeric, DistString},
    Rng,
};
use reqwest::header::HeaderMap;

use crate::utils::constants::DsSalt;

use super::{
    common::{headermap, Region},
    constants::Lang,
};

pub(crate) fn generate_dynamic_secret(salt: DsSalt) -> String {
    let t = Utc::now().timestamp();
    let r: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 6);
    let hash = md5::compute(format!("salt={}&t={}&r={}", salt.val(), t, r));

    format!("{},{},{:x}", t, r, hash)
}

pub(crate) fn generate_cn_dynamic_secret(
    body: &str,
    query: &HashMap<String, String>,
    salt: DsSalt,
) -> String {
    let t = Utc::now().timestamp();
    let r: u32 = rand::thread_rng().gen_range(100001..=200000);
    let q = query
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join("&");

    let hash = md5::compute(format!(
        "salt={}&t={}&r={}&b={}&q={}",
        salt.val(),
        t,
        r,
        body,
        q
    ));

    format!("{},{},{:x}", t, r, hash)
}

pub(crate) fn get_headers(lang: Lang) -> HeaderMap {
    headermap! {
        "x-rpc-app_version" => "1.5.0",
        "x-rpc-client_type" => "5",
        "x-rpc-language" => lang.val(),
        "x-rpc-lang" => lang.val(),
        "ds" => &generate_dynamic_secret(DsSalt::Overseas),
    }
}

pub(crate) fn get_cn_headers(
    data: Option<&str>,
    query: Option<&HashMap<String, String>>,
) -> HeaderMap {
    let query = if query.is_some() {
        query.unwrap()
    } else {
        &HashMap::new()
    };

    let data = data.unwrap_or("");
    headermap! {
        "x-rpc-app_version" => "2.11.1",
        "x-rpc-client_type" => "5",
        "ds" => &generate_cn_dynamic_secret(data, query, DsSalt::Chinese),
    }
}

pub(crate) fn generate_passport_ds(body: &str) -> String {
    let t = Utc::now().timestamp();
    let r: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 6);
    let hash = md5::compute(format!(
        "salt={}&t={}&r={}&b={}&q=",
        DsSalt::CnPassport.val(),
        t,
        r,
        body
    ));

    format!("{},{},{:x}", t, r, hash)
}

pub(crate) fn generate_geetest_ds(region: Region) -> String {
    let t = Utc::now().timestamp();
    let r: u32 = rand::thread_rng().gen_range(100000..=200000);
    let hash = md5::compute(format!(
        "salt={}&t={}&r={}&b=&q=is_high=false",
        DsSalt::from(region).val(),
        t,
        r
    ));

    format!("{},{},{:x}", t, r, hash)
}
