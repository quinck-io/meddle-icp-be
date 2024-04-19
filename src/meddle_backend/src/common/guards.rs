use candid::Principal;
use ic_cdk::caller;

pub fn caller_is_user() -> Result<(), String> {
    let caller = caller();
    if caller == Principal::anonymous() {
        return Err("Caller is not the user of the canister.".to_string());
    }
    Ok(())
}
