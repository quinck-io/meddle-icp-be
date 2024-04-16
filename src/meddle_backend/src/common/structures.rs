use std::borrow::Cow;

use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};

pub struct SingleData {
    sensorId: String,
    value: f32,
    timestamp: u64,
    timestampString: String,
}

pub struct JsonData {
    endpoint: String,
    variables: Vec<SingleData>,
}

pub struct Data {
    input_id: String,
    sensor_id: String,
    value: f32,
    timestamp: u64,
}
