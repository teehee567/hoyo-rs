use std::sync::LazyLock;

use base64::{engine::general_purpose, prelude::BASE64_STANDARD, Engine};
use reqwest::header::HeaderMap;
use rsa::{pkcs8::DecodePublicKey, traits::PaddingScheme, Pkcs1v15Encrypt, RsaPublicKey};
use serde::Serialize;

use crate::{utils::common::headermap, HoyoError};

use super::common::Region;

pub(crate) fn hoyo_encrypt(data: &str, region: Region) -> String {
    let public_key = RsaPublicKey::from_public_key_pem(get_rsa_key(region))
        .expect("Could not decode Public Key");

    let mut rng = rand::thread_rng();
    let enc = public_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, data.as_bytes())
        .expect("Failed to encrypt");

    BASE64_STANDARD.encode(&enc)
}

const OS_LOGIN_RSA_KEY: &'static str = "
-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA4PMS2JVMwBsOIrYWRluY
wEiFZL7Aphtm9z5Eu/anzJ09nB00uhW+ScrDWFECPwpQto/GlOJYCUwVM/raQpAj
/xvcjK5tNVzzK94mhk+j9RiQ+aWHaTXmOgurhxSp3YbwlRDvOgcq5yPiTz0+kSeK
ZJcGeJ95bvJ+hJ/UMP0Zx2qB5PElZmiKvfiNqVUk8A8oxLJdBB5eCpqWV6CUqDKQ
KSQP4sM0mZvQ1Sr4UcACVcYgYnCbTZMWhJTWkrNXqI8TMomekgny3y+d6NX/cFa6
6jozFIF4HCX5aW8bp8C8vq2tFvFbleQ/Q3CU56EWWKMrOcpmFtRmC18s9biZBVR/
8QIDAQAB
-----END PUBLIC KEY-----
";

const CN_LOGIN_RSA_KEY: &'static str = "
-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDDvekdPMHN3AYhm/vktJT+YJr7
cI5DcsNKqdsx5DZX0gDuWFuIjzdwButrIYPNmRJ1G8ybDIF7oDW2eEpm5sMbL9zs
9ExXCdvqrn51qELbqj0XxtMTIpaCHFSI50PfPpTFV9Xt/hmyVwokoOXFlAEgCn+Q
CgGs52bFoYMtyi+xEQIDAQAB
-----END PUBLIC KEY-----
";

pub(crate) const fn get_rsa_key(region: Region) -> &'static str {
    match region {
        Region::Overseas => OS_LOGIN_RSA_KEY,
        Region::Chinese => OS_LOGIN_RSA_KEY,
    }
}

pub(crate) static WEB_LOGIN_HEADERS: LazyLock<HeaderMap> = {
    LazyLock::new(|| {
        headermap! {
            "x-rpc-app_id" => "c9oqaq3s3gu8",
            "x-rpc-client_type" => "4",
            // If not equal account.hoyolab.com It's will return retcode 1200 [Unauthorized]
            "Origin" => "https://account.hoyolab.com",
            "Referer" => "https://account.hoyolab.com/",
        }
    })
};

pub(crate) static APP_LOGIN_HEADERS: LazyLock<HeaderMap> = {
    LazyLock::new(|| {
        headermap! {
        "x-rpc-app_id" => "c9oqaq3s3gu8",
        "x-rpc-client_type" => "2",
        // Passing "x-rpc-device_id" header will trigger email verification
        // (unless the device_id is already verified).
        //
        // For some reason, without this header, email verification is not triggered.
        // "x-rpc-device_id": "1c33337bd45c1bfs",
            }
    })
};

pub(crate) static CN_LOGIN_HEADERS: LazyLock<HeaderMap> = {
    LazyLock::new(|| {
        headermap! {
            "x-rpc-app_id" => "bll8iq97cem8",
            "x-rpc-client_type" => "4",
            "x-rpc-source" => "v2.webLogin",
            "x-rpc-device_fp" => "38d7fff8fd68c",
            "x-rpc-device_id" => "469af8a4-0754-4a3c-a999-dec592f00894",
            "x-rpc-device_model" => "Firefox%20131.0",
            "x-rpc-device_name" => "Firefox",
            "x-rpc-game_biz" => "bbs_cn",
            "x-rpc-sdk_version" => "2.31.0",
        }
    })
};

pub(crate) static EMAIL_SEND_HEADERS: LazyLock<HeaderMap> = {
    LazyLock::new(|| {
        headermap! {
            "x-rpc-app_id" => "c9oqaq3s3gu8",
            "x-rpc-client_type" => "2",
        }
    })
};

pub(crate) static EMAIL_VERIFY_HEADERS: LazyLock<HeaderMap> = {
    LazyLock::new(|| {
        headermap! {
            "x-rpc-app_id" => "c9oqaq3s3gu8",
            "x-rpc-client_type" => "2",
        }
    })
};

pub(crate) static QRCODE_HEADERS: LazyLock<HeaderMap> = {
    LazyLock::new(|| {
        headermap! {
            "x-rpc-app_id" => "bll8iq97cem8",
            "x-rpc-client_type" => "4",
            "x-rpc-game_biz" => "bbs_cn",
            "x-rpc-device_fp" => "38d7fa104e5d7",
            "x-rpc-device_id" => "586f1440-856a-4243-8076-2b0a12314197",
        }
    })
};

pub(crate) static OS_MMT_HEADERS: LazyLock<HeaderMap> = {
    LazyLock::new(|| {
        headermap! {
            "x-rpc-challenge_path" => "https://bbs-api-os.hoyolab.com/game_record/app/hkrpg/api/challenge",
            "x-rpc-app_version" => "2.55.0",
            "x-rpc-challenge_game" => "6",
            "x-rpc-client_type" => "5",
        }
    })
};

pub(crate) static CN_MMT_HEADERS: LazyLock<HeaderMap> = {
    LazyLock::new(|| {
        headermap! {
            "x-rpc-app_version" => "2.60.1",
            "x-rpc-client_type" => "5",
            "x-rpc-challenge_game" => "6",
            "x-rpc-page" => "v1.4.1-rpg_#/rpg",
            "x-rpc-tool-version" => "v1.4.1-rpg",
        }
    })
};

pub(crate) const DEVICE_ID: &'static str = "D6AF5103-D297-4A01-B86A-87F87DS5723E";

pub(crate) static RISKY_CHECK_HEADERS: LazyLock<HeaderMap> = {
    LazyLock::new(|| {
        headermap! {
            "x-rpc-client_type" => "1",
            "x-rpc-channel_id" => "1",
        }
    })
};

pub(crate) static SHIELD_LOGIN_HEADERS: LazyLock<HeaderMap> = {
    LazyLock::new(|| {
        headermap! {
            "x-rpc-client_type" => "1",
            "x-rpc-channel_id" => "1",
            "x-rpc-device_id" => DEVICE_ID,
        }
    })
};

pub(crate) static GRANT_TICKET_HEADERS: LazyLock<HeaderMap> = {
    LazyLock::new(|| {
        headermap! {
            "x-rpc-client_type" => "1",
            "x-rpc-channel_id" => "1",
            "x-rpc-device_id" => DEVICE_ID,
            "x-rpc-language" => "en",
        }
    })
};

pub(crate) static GAME_LOGIN_HEADERS: LazyLock<HeaderMap> = {
    LazyLock::new(|| {
        headermap! {
            "x-rpc-client_type" => "1",
            "x-rpc-channel_id" => "1",
            "x-rpc-device_id" => DEVICE_ID,
        }
    })
};
