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
