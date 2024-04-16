use std::borrow::Cow;

use candid::CandidType;
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};

#[derive(Debug, CandidType, Deserialize)]
#[allow(non_snake_case)]
pub struct SingleInput {
    pub sensorId: String,
    pub value: f32,
    pub timestamp: u64,
    pub timestampString: String,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct JsonInput {
    pub endpoint: String,
    pub variables: Vec<SingleInput>,
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct Data {
    pub unit_id: String,
    pub sensor_id: String,
    pub value: f32,
    pub timestamp: u64,
}

impl Storable for Data {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(serde_json::to_string(self).unwrap().as_bytes().to_vec())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        serde_json::from_str(String::from_utf8(bytes.to_vec()).unwrap().as_str()).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 1024,
        is_fixed_size: false,
    };
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct OperationResult {
    pub unit_id: String,
    pub code: u16,
    pub message: String,
}
