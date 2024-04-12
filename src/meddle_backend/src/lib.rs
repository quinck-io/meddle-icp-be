pub mod common;
pub mod api;
pub mod database;

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
