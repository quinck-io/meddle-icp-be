use std::borrow::Cow;

use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct SingleData {
    sensorId: String,
    value: f32,
    timestamp: u64,
    timestampString: String,
}

#[derive(Debug)]
pub struct JsonData {
    endpoint: String,
    variables: Vec<SingleData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    input_id: String,
    sensor_id: String,
    value: f32,
    timestamp: u64,
}

impl Storable for Data {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(serde_json::to_string(self).unwrap().as_bytes().to_vec())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        serde_json::from_str(String::from_utf8(bytes.to_vec()).unwrap().as_str()).unwrap()
    }

    const BOUND: ic_stable_structures::storable::Bound = Bound::Bounded {
        max_size: 1024,
        is_fixed_size: false,
    };
}
