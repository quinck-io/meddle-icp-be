use crate::{
    common::structures::{Comparator, Data, OperationResult, OutUnitId},
    database::get_records,
};
use itertools::Itertools;

pub fn get_data_by_sensor(sensor: String, offset: u32, limit: u32, from_latest: bool) -> Vec<Data> {
    let mut sensor_records: Vec<Data> = get_records();

    if from_latest {
        sensor_records.reverse();
    }

    sensor_records
        .iter()
        .filter(|data| data.sensor_id.contains(&sensor))
        .skip(offset as usize)
        .take(limit as usize)
        .map(|elem| elem.clone())
        .collect()
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
    from_latest: bool,
) -> Vec<Data> {
    get_data_by_sensor(sensor, offset, limit, from_latest)
        .iter()
        .filter(|data| compare(comparator, data.value, value))
        .map(|data| data.clone())
        .collect()
}

pub fn get_data_by_range(
    start: u64,
    end: Option<u64>,
    offset: u32,
    limit: u32,
    from_latest: bool,
) -> Vec<Data> {
    let mut records = get_records();

    if !from_latest {
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

pub fn get_data(offset: u32, limit: u32, from_latest: bool) -> (Vec<Data>, u32) {
    let mut records = get_records();

    if !from_latest {
        records.reverse();
    }

    (
        records
            .iter()
            .skip(offset as usize)
            .take(limit as usize)
            .map(|x| x.clone())
            .collect::<Vec<Data>>(),
        records.len() as u32,
    )
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

pub fn get_all_unit_ids(offset: u32, limit: u32, from_latest: bool) -> OutUnitId {
    let mut records: Vec<Data> = get_records();
    let length = records.len() as u32;
    if from_latest {
        records.reverse();
    }
    OutUnitId {
        unit_ids: records
            .clone()
            .iter()
            .map(|data| data.unit_id.clone())
            .unique()
            .skip(offset as usize)
            .take(limit as usize)
            .collect(),
        len: length,
    }
}
