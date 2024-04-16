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
