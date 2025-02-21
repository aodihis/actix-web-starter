use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct ExampleRequest {
    pub id: String,
}

#[derive(Serialize)]
pub struct ExampleResponse {
    pub id: String,
}