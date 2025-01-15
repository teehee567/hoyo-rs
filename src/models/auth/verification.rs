use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize, Debug)]
pub struct ActionTicket {
    risk_ticket: String,
    verify_str: String,
    pub(crate) ticket: String,
}

impl ActionTicket {
    pub fn to_rpc_verify_header(&self) -> String {
        let verify_str_value =
            serde_json::from_str::<Value>(&self.verify_str).unwrap_or_else(|_| json!({}));
        let mut ticket_json = json!({
            "risk_ticket": self.risk_ticket,
            "verify_str": verify_str_value,
            "ticket": self.ticket,
        });
        let verify_str_as_string = verify_str_value.to_string();
        ticket_json["verify_str"] = Value::String(verify_str_as_string);
        serde_json::to_string(&ticket_json).unwrap()
    }
}
