use crate::{
    common::structures::{Comparator, Data, OperationResult},
    database::get_records,
};

pub fn get_data_by_sensor(sensor: String, offset: u32, limit: u32, from_recent: bool) -> Vec<Data> {
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
    return sensor_records.clone();
}

fn compare(comparator: Comparator, to_compare: f32, fixed_value: f32) -> bool {
    match comparator {
        Comparator::GREATER => to_compare > fixed_value,
        Comparator::MINUS => to_compare < fixed_value,
        Comparator::EQUALS => to_compare == fixed_value,
    }
}

pub fn get_data_by_sensor_filter(
    sensor: String,
    value: f32,
    comparator: Comparator,
    offset: u32,
    limit: u32,
    from_recent: bool,
) -> Vec<Data> {
    get_data_by_sensor(sensor, offset, limit, from_recent)
        .iter()
        .filter(|data| compare(comparator, data.value, value))
        .map(|data| data.clone())
        .collect()
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
            .filter(|x| x.timestamp > start && x.timestamp > end)
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

    if records.is_empty() {
        return Err(OperationResult {
            unit_id: [unit_id].to_vec(),
            code: 404,
            message: String::from("Element Not Found"),
        });
    }

    Ok(records)
}
