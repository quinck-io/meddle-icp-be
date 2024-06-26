use candid::Principal;
use common::guards::caller_is_user;
use common::structures::{Comparator, Data, JsonInput, OperationResult, OutDataRecords, OutUnitId};
use ic_cdk::caller;

pub mod api;
pub mod common;
pub mod database;

/// Standard greet function to check the connection
///
/// ## Arguments
/// * `name` - name to greet
///
/// ## Returns
/// * a greet message as string
#[ic_cdk::query(guard = "caller_is_user")]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

/// Standard whoami to retrieve the principal
///
/// ## Returns
/// * principal of the caller
#[ic_cdk::query(guard = "caller_is_user")]
fn whoami() -> Principal {
    ic_cdk::println!("{}", caller());
    caller()
}

/// Add data to the database
///
/// ## Arguments
///
/// * `vec_json_data` - Json containing an array of elements in which there are two fields:
///     Endpoint and Variables. <br>
///     Variables needs to be a vec of json containig the actual values that are then added to the database.
///
/// ### Example of a good input:
/// ```rust
/// {
///  "modbus-input": [
///    {
///      "endpoint": "192.168.101.16",
///      "variables": [
///        {
///          "sensorId": "Tensione L1-N",
///          "value": 236.91445922851562,
///          "timestamp": 1713166215437,
///          "timestampString": "2024-04-15 07:30:15",
///          "payload": null
///        },
///        {
///          "sensorId": "Tensione L2-N",
///          "value": 232.7878875732422,
///          "timestamp": 1713166215496,
///          "timestampString": "2024-04-15 07:30:15",
///          "payload": null
///        },
///        {
///          "sensorId": "Tensione L3-N",
///          "value": 251.20216369628906,
///          "timestamp": 1713166215576,
///          "timestampString": "2024-04-15 07:30:15",
///          "payload": null
///        },
///        {
///          "sensorId": "Corrente L1",
///          "value": 0.06031423434615135,
///          "timestamp": 1713166215631,
///          "timestampString": "2024-04-15 07:30:15",
///          "payload": null
///        },
///        {
///          "sensorId": "Corrente L2",
///          "value": 0.041959624737501144,
///          "timestamp": 1713166215687,
///          "timestampString": "2024-04-15 07:30:15",
///          "payload": null
///        },
///        {
///          "sensorId": "Corrente L3",
///          "value": 0.040930937975645065,
///          "timestamp": 1713166215753,
///          "timestampString": "2024-04-15 07:30:15",
///          "payload": null
///        },
///        {
///          "sensorId": "Potenza apparente L1",
///          "value": 14.275617599487305,
///          "timestamp": 1713166215806,
///          "timestampString": "2024-04-15 07:30:15",
///          "payload": null
///        },
///        {
///          "sensorId": "Potenza apparente L2",
///          "value": 9.757477760314941,
///          "timestamp": 1713166215864,
///          "timestampString": "2024-04-15 07:30:15",
///          "payload": null
///        },
///        {
///          "sensorId": "Potenza apparente L3",
///          "value": 10.279207229614258,
///          "timestamp": 1713166215918,
///          "timestampString": "2024-04-15 07:30:15",
///          "payload": null
///        }
///          "timestamp": 1713166215864,
///          "timestampString": "2024-04-15 07:30:15",
///          "payload": null
///        },
///        {
///          "sensorId": "Potenza apparente L3",
///          "value": 10.279207229614258,
///          "timestamp": 1713166215918,
///          "timestampString": "2024-04-15 07:30:15",
///          "payload": null
///        }
///      ]
///    }
///  ]
///}
/// ```
///
/// ## Return
///
/// * OperationResult containing:
///     * A vector of unit_id, which are the ids of the units that where inserted correctly
///     * Code 400 if there where any error in adding the data
///     * A message for more information about the result
#[ic_cdk::update(guard = "caller_is_user")]
fn post_data(vec_json_data: Vec<JsonInput>) -> OperationResult {
    crate::api::create::post_data(vec_json_data)
}

/// Get a unit Record <br>
/// Every element in the database is part of a unit having a unique id, this API returns a vector of data of that unit
///
/// ## Arguments
/// * `unit_id` - The unique id of the unit you want as return
///
/// ## Returns either
/// * Vector of data with that unique unit_id
/// * OperationResult in case of error containing:
///     * A vector having only the unit_id not found
///     * Code 404
///     * A message for more information about the error
#[ic_cdk::query(guard = "caller_is_user")]
fn get_record(unit_id: String) -> Result<Vec<Data>, OperationResult> {
    crate::api::read::get_record(unit_id)
}

/// Retrive all elements from the database
///
/// ## Arguments
/// * `offset` - Offset of the first element to retrieve
/// * `limit` - Number of elements to retrieve
/// * `from_latest` - Flag to set the output order of the data
///
/// ## Returns
/// * Vector of data and the length of the total records
#[ic_cdk::query(guard = "caller_is_user")]
fn get_data(offset: u32, limit: u32, from_latest: bool) -> OutDataRecords {
    crate::api::read::get_data(offset, limit, from_latest)
}

/// Retrieve all the UnitIDs of all the records
///
/// ## Arguments
/// * `offset` - Offset of the first element to retrieve
/// * `limit` - Number of elements to retrieve
/// * `from_latest` - Flag to set the output order of the data
///
/// ## Returns
/// Vector containig all the unit ids and the len of all the records
#[ic_cdk::query(guard = "caller_is_user")]
fn get_all_unit_ids(offset: u32, limit: u32, from_latest: bool) -> OutUnitId {
    crate::api::read::get_all_unit_ids(offset, limit, from_latest)
}

/// Retrieve all the datas from a vec of ids
///
/// ## Arguments
/// * `ids` - Vector of ids
/// * `offset` - Offset of the first element to retrieve
/// * `limit` - Number of elements to retrieve
/// * `from_latest` - Flag to set the output order of the data
///
/// ## Returns
/// Vector containig all the datas and the len of all the records
#[ic_cdk::query(guard = "caller_is_user")]
fn get_data_by_multiple_ids(
    ids: Vec<String>,
    offset: u32,
    limit: u32,
    from_latest: bool,
) -> Result<OutDataRecords, OperationResult> {
    crate::api::read::get_data_by_multiple_ids(ids, offset, limit, from_latest)
}

/// Retrive all elements from the database between a specific start time to an end
///
/// ## Arguments
/// * `start` - Timestamp in nanoseconds of the start time from which the data needs to be retrieved
/// * `end` - Optional timestamp in nanoseconds of the end from which the data needs to be retrieved (if end in null then it is considered now as the end time)
/// * `offset` - Offset of the first element to retrieve
/// * `limit` - Number of elements to retrieve
/// * `from_latest` - Flag to set the output order of the data
///
/// ## Returns
/// * Vector of data and the length of the total filtered values
#[ic_cdk::query(guard = "caller_is_user")]
fn get_data_by_range(
    start: u64,
    end: Option<u64>,
    offset: u32,
    limit: u32,
    from_latest: bool,
) -> OutDataRecords {
    crate::api::read::get_data_by_range(start, end, offset, limit, from_latest)
}

/// Get all data sent by a specific sensor
/// ## Arguments
///
/// * `sensor` - Sensor identifier
/// * `offset` - Offset of the first element to retrieve
/// * `limit` - Number of elements to retrieve
/// * `from_latest` - Flag to change the output order of the data
///
/// ## Return
/// * Vector containing all data logged from the sensor given in input and the length of the total filtered values
#[ic_cdk::query(guard = "caller_is_user")]
fn get_data_by_sensor(
    sensor: String,
    offset: u32,
    limit: u32,
    from_latest: bool,
) -> OutDataRecords {
    crate::api::read::get_data_by_sensor(sensor, offset, limit, from_latest)
}

/// Get all data sent by a specific sensor
/// ## Arguments
///
/// * `sensor` - Sensor identifier
/// * `value` - Value that has to be comparated with the data values
/// * `comparator` - String representing the compare to do
/// * `offset` - Offset of the first element to retrieve
/// * `limit` - Number of elements to retrieve
/// * `from_latest` - Flag to set the output order of the data
///
/// ## Admissible values
/// comparator should be one of the following values: >, <, =
///
/// ## Return
/// * Vector containing all data logged from the sensor given in input and the length of the total filtered values
/// * OperationResult in the case the comparator is not well formatted
#[ic_cdk::query(guard = "caller_is_user")]
fn get_data_by_sensor_filter(
    sensor: String,
    value: f32,
    comparator: String,
    offset: u32,
    limit: u32,
    from_latest: bool,
) -> Result<OutDataRecords, OperationResult> {
    let comparator = match Comparator::from_string(comparator) {
        Ok(val) => val,
        Err(e) => {
            return Err(e);
        }
    };

    Ok(crate::api::read::get_data_by_sensor_filter(
        sensor,
        value,
        comparator,
        offset,
        limit,
        from_latest,
    ))
}
