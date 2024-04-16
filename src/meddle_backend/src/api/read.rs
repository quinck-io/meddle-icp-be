use crate::{
    common::structures::{Comparator, Data, OperationResult, OutDataRecords},
    database::get_records,
};

pub fn get_data_by_sensor(sensor: String, offset: u32, limit: u32, from_latest: bool) -> OutDataRecords {
    let mut sensor_records: Vec<Data> = get_records();

    if from_latest {
        sensor_records.reverse();
    }

    let sensor_records = sensor_records
        .iter()
        .filter(|data| data.sensor_id.contains(&sensor))
        .map(|elem| elem.clone())
        .collect::<Vec<Data>>();

    OutDataRecords {
        data: sensor_records
            .iter()
            .skip(offset as usize)
            .take(limit as usize)
            .map(|x| x.clone())
            .collect::<Vec<Data>>(),
        len: sensor_records.len() as u32
    }
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
) -> OutDataRecords {
    let mut sensor_records: Vec<Data> = get_records();

    if from_latest {
        sensor_records.reverse();
    }

    let sensor_records = sensor_records
        .iter()
        .filter(|data| data.sensor_id.contains(&sensor))
        .filter(|data| compare(comparator, data.value, value))
        .map(|elem| elem.clone())
        .collect::<Vec<Data>>();
    
    OutDataRecords {
        data: sensor_records
            .iter()
            .skip(offset as usize)
            .take(limit as usize)
            .map(|x| x.clone())
            .collect::<Vec<Data>>(),
        len: sensor_records.len() as u32
    }
}

pub fn get_data_by_range(
    start: u64,
    end: Option<u64>,
    offset: u32,
    limit: u32,
    from_latest: bool,
) -> OutDataRecords{
    let mut records = get_records();

    if !from_latest {
        records.reverse();
    }

    let records = match end {
        Some(end) => records
            .iter()
            .filter(|x| x.timestamp > start && x.timestamp < end)
            .map(|x| x.clone())
            .collect::<Vec<Data>>(),
        None => records
            .iter()
            .filter(|x| x.timestamp > start)
            .map(|x| x.clone())
            .collect::<Vec<Data>>(),
    };

    OutDataRecords {
        data: records
            .iter()
            .skip(offset as usize)
            .take(limit as usize)
            .map(|x| x.clone())
            .collect::<Vec<Data>>(),
        len: records.len() as u32
    }
}

pub fn get_data(offset: u32, limit: u32, from_latest: bool) -> OutDataRecords {
    let mut records = get_records();

    if !from_latest {
        records.reverse();
    }

    OutDataRecords {
        data: records
            .iter()
            .skip(offset as usize)
            .take(limit as usize)
            .map(|x| x.clone())
            .collect::<Vec<Data>>(),
        len: records.len() as u32,
    }
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
