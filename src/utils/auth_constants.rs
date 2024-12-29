use std::sync::LazyLock;

use reqwest::header::HeaderMap;

use crate::utils::common::headermap;

pub(crate) const LOGIN_KEY_TYPE_1: &'static [u8; 452] = b"
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

pub(crate) const LOGIN_KEY_TYPE_2: &'static [u8; 273] = b"
-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDDvekdPMHN3AYhm/vktJT+YJr7
cI5DcsNKqdsx5DZX0gDuWFuIjzdwButrIYPNmRJ1G8ybDIF7oDW2eEpm5sMbL9zs
9ExXCdvqrn51qELbqj0XxtMTIpaCHFSI50PfPpTFV9Xt/hmyVwokoOXFlAEgCn+Q
CgGs52bFoYMtyi+xEQIDAQAB
-----END PUBLIC KEY-----
";



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
