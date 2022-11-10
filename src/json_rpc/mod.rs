use core::panic;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JSONRPCPayload {
    method: String,
    params: Vec<String>,
}

pub enum JSONRPCMethod {
    // Network, epoch
    OrandGetPublicEpoch(i32, i32),
    // New epoch of given network
    OrandNewEpoch(i32),
}

impl JSONRPCMethod {
    pub fn from_json_string(json_string: &str) -> Self {
        let json_rpc: JSONRPCPayload = serde_json::from_str(json_string).unwrap();
        match json_rpc.method.as_str() {
            "orand_getPublicEpoch" => Self::OrandGetPublicEpoch(
                json_rpc.params[0].as_str().parse().unwrap(),
                json_rpc.params[1].as_str().parse().unwrap(),
            ),
            "orand_newEpoch" => Self::OrandNewEpoch(json_rpc.params[0].as_str().parse().unwrap()),
            _ => panic!("Unsupported method"),
        }
    }
}
