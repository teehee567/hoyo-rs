use serde::Deserialize;
use serde_json::Value;


#[derive(Deserialize)]
pub(crate) struct Base {
    pub retcode: i32,
    pub message: String,
    pub data: Option<Value>,
}
