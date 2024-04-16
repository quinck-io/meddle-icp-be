use ic_stable_structures::log::WriteError;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, Log};

use std::cell::RefCell;

use crate::common::structures::Data;

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {

    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static SENSOR_RECORDS: RefCell<Log<Data, Memory, Memory>> = RefCell::new({
        Log::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
        ).expect("failed to initialize stable log")
    });
}

pub fn get_records() -> Vec<Data> {
    SENSOR_RECORDS.with(|x| x.borrow().iter().collect::<Vec<Data>>())
}

pub fn insert_record(data: Data) -> Result<u64, WriteError> {
    SENSOR_RECORDS.with(|x| x.borrow_mut().append(&data))
}
