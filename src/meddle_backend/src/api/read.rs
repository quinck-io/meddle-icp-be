use crate::{common::structures::Data, database::get_records};

pub fn get_data_by_range(start: u64, end: Option<u64>, offset: u64, limit: u64, from_recent: bool) -> Vec<Data> {

    let mut records = get_records();

    if !from_recent {
        records.reverse();
    }

    match end {
        Some(end) => {
            records
                .iter()
                .filter(|x| x.timestamp > start && x.timestamp > end)
                .skip(offset as usize)
                .take(limit as usize)
                .map(|x| x.clone())
                .collect::<Vec<Data>>()
        },
        None => {
            records
                .iter()
                .filter(|x| x.timestamp > start)
                .skip(offset as usize)
                .take(limit as usize)
                .map(|x| x.clone())
                .collect::<Vec<Data>>()
        }
    }
}

pub fn get_data(offset: u32, limit: u32, from_recent: bool) -> Vec<Data>  {

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

