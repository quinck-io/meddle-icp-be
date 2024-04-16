use common::structures::{Data, JsonInput, OperationResult};

pub mod api;
pub mod common;
pub mod database;

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

/// .
#[ic_cdk::update]
fn post_data(vec_json_data: Vec<JsonInput>) -> OperationResult {
    crate::api::create::post_data(vec_json_data)
}

/// .
#[ic_cdk::query]
fn get_record(unit_id: String) -> Result<Vec<Data>, OperationResult> {
    crate::api::read::get_record(unit_id)
}

/// .
#[ic_cdk::query]
fn get_data(offset: u32, limit: u32, from_recent: bool) -> Vec<Data> {
    crate::api::read::get_data(offset, limit, from_recent)
}

/// .
#[ic_cdk::query]
fn get_data_by_range(
    start: u64,
    end: Option<u64>,
    offset: u64,
    limit: u64,
    from_recent: bool,
) -> Result<Vec<Data>, OperationResult> {
    Ok(crate::api::read::get_data_by_range(
        start,
        end,
        offset,
        limit,
        from_recent,
    ))
}

/// .
#[ic_cdk::query]
fn get_data_by_sensor(
    sensor: String,
    offset: u32,
    limit: u32,
    from_recent: bool,
) -> Result<Vec<Data>, OperationResult> {
    crate::api::read::get_data_by_sensor(sensor, offset, limit, from_recent)
}

/// .
#[ic_cdk::query]
fn get_data_by_sensor_filter(
    sensor: String,
    value: f32,
    comparator: String,
    offset: u32,
    limit: u32,
    from_recent: bool,
) -> Result<Vec<Data>, OperationResult> {
    crate::api::read::get_data_by_sensor_filter(
        sensor,
        value,
        comparator,
        offset,
        limit,
        from_recent,
    )
}
