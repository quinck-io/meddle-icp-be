use crate::{common::{structures::{Data, JsonInput, OperationResult}, uuid::uuid_v4}, database::insert_record};

pub fn post_data(vec_json_data: Vec<JsonInput>) -> OperationResult {

    let mut ids: Vec<String> = Vec::new();
    let mut code = 200;
    let mut local_timestamp = Option::None;
    vec_json_data
        .iter()
        .for_each(|x| {
            
            let unit_id = uuid_v4(local_timestamp);
            ids.push(unit_id.clone());

            x.variables
                .iter()
                .for_each(|input| {
                
                    let res = insert_record( Data {
                        unit_id: unit_id.clone(),
                        sensor_id: input.sensorId.clone(),
                        value: input.value,
                        timestamp: input.timestamp
                    });
                
                    if res.is_err() {
                        code = 400;
                        return;
                    }

                    local_timestamp = Option::from(input.timestamp);
                });

            if code == 400 {
                return;
            }
        });

    if code == 400 {
        return OperationResult {unit_id: ids, code, message: String::from("Error in writing elements in database")};
    }
    OperationResult {unit_id: ids, code, message: String::from("All Element Inserted Correctly")}
}