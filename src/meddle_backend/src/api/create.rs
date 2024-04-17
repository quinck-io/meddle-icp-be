use crate::{common::{structures::{Data, JsonInput, OperationResult}, uuid::uuid_v4}, database::insert_record};

pub fn post_data(vec_json_data: Vec<JsonInput>) -> OperationResult {

    let mut ids: Vec<String> = Vec::new();
    let mut local_timestamp = Option::None;
    for x in vec_json_data {  
            let unit_id = uuid_v4(local_timestamp);
            ids.push(unit_id.clone());

            for input in x.variables {

                    let res = insert_record( Data {
                        unit_id: unit_id.clone(),
                        sensor_id: input.sensorId.clone(),
                        value: input.value,
                        timestamp: input.timestamp
                    });
                
                    if res.is_err() {
                        return OperationResult {unit_id: ids, code: 400, message: String::from("Error in writing elements in database")};
                    }

                    local_timestamp = Option::from(input.timestamp);
                }
        }

    OperationResult {unit_id: ids, code: 200, message: String::from("All Element Inserted Correctly")}
}