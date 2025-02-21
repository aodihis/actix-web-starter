use actix_web::{web, HttpResponse};
use validator::Validate;
use crate::errors::api_error::ApiError;
use crate::models::example::{ExampleRequest, ExampleResponse};

pub async fn example_handler(data: web::Json<ExampleRequest>) -> Result<HttpResponse, ApiError> {
    if let Err(err) = data.validate() {
        return Err(ApiError::ValidationError(err.to_string()));
    }

    Ok(HttpResponse::Ok().json(
        ExampleResponse {
            id: data.id.clone(),
        }
    ))
}