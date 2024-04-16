use crate::{
    common::structures::{Data, OperationResult},
    database::get_records,
};

pub fn get_data_by_sensor(
    sensor: String,
    offset: u32,
    limit: u32,
    from_recent: bool,
) -> Result<Vec<Data>, OperationResult> {
    let mut sensor_records: Vec<Data> = get_records()
        .iter()
        .filter(|data| data.sensor_id.contains(&sensor))
        .skip(offset as usize)
        .take(limit as usize)
        .map(|elem| elem.clone())
        .collect();
    if !from_recent {
        sensor_records.reverse();
    }
    return Ok(sensor_records.clone());
}

pub fn get_data_by_sensor_filter(
    sensor: String,
    value: f32,
    comparator: String,
    offset: u32,
    limit: u32,
    from_recent: bool,
) -> Result<Vec<Data>, OperationResult> {
    match get_data_by_sensor(sensor, offset, limit, from_recent) {
        Ok(vector) => match comparator.as_str() {
            ">" => {
                return Ok(vector
                    .iter()
                    .filter(|data| data.value > value)
                    .map(|data| data.clone())
                    .collect::<Vec<Data>>())
            }
            _ => return Ok(vector),
        },
        Err(e) => Err(e),
    }
}

pub fn get_data_by_range(
    start: u64,
    end: Option<u64>,
    offset: u64,
    limit: u64,
    from_recent: bool,
) -> Vec<Data> {
    let mut records = get_records();

    if !from_recent {
        records.reverse();
    }

    match end {

        Some(end) => records
            .iter()
            .filter(|x| x.timestamp > start && x.timestamp < end)
            .skip(offset as usize)
            .take(limit as usize)
            .map(|x| x.clone())
            .collect::<Vec<Data>>(),

        None => records
            .iter()
            .filter(|x| x.timestamp > start)
            .skip(offset as usize)
            .take(limit as usize)
            .map(|x| x.clone())
            .collect::<Vec<Data>>(),
    }
}

pub fn get_data(offset: u32, limit: u32, from_recent: bool) -> Vec<Data> {
    let mut records = get_records();

    if !from_recent {
        records.reverse();
    }

    records
        .iter()
        .skip(offset as usize)
        .take(limit as usize)
        .map(|x| x.clone())
        .collect::<Vec<Data>>()
}

pub fn get_record(unit_id: String) -> Result<Vec<Data>, OperationResult> {
    let records = get_records()
        .iter()
        .filter(|x| x.unit_id == unit_id)
        .map(|x| x.clone())
        .collect::<Vec<Data>>();

    if records.is_empty() { return  Err(OperationResult {unit_id: [unit_id].to_vec(), code: 404, message: String::from("Element Not Found")}) }

    Ok(records)
}
