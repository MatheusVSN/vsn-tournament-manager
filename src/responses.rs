use std::io::Cursor;

use rocket::{
    http::{ContentType, Status},
    response::{self, Responder},
    Response,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomResponse {
    pub message: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone)]
pub enum HTTPException {
    Internal(String),
    NotFound(String),
    BadRequest(String),
    Conflict(String),
    Unauthorized(String),
}

#[derive(Debug, Clone)]
pub enum HTTPSuccessResponse {
    Ok(CustomResponse),
    Created(CustomResponse),
}

impl HTTPException {
    fn get_http_status(&self) -> Status {
        match self {
            HTTPException::Internal(_) => Status::InternalServerError,
            HTTPException::NotFound(_) => Status::NotFound,
            HTTPException::BadRequest(_) => Status::BadRequest,
            HTTPException::Conflict(_) => Status::Conflict,
            HTTPException::Unauthorized(_) => Status::Unauthorized,
        }
    }
}

impl HTTPSuccessResponse {
    fn get_http_status(&self) -> Status {
        match self {
            HTTPSuccessResponse::Ok(_) => Status::Ok,
            HTTPSuccessResponse::Created(_) => Status::Created,
        }
    }

    fn get_http_response(&self) -> CustomResponse {
        match self {
            HTTPSuccessResponse::Ok(response) => response.clone(),
            HTTPSuccessResponse::Created(response) => response.clone(),
        }
    }
}

impl std::fmt::Display for HTTPException {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let error_message = match self {
            Self::BadRequest(message) => message,
            Self::Conflict(message) => message,
            Self::Internal(message) => message,
            Self::NotFound(message) => message,
            Self::Unauthorized(message) => message,
        };

        write!(fmt, "{}", error_message)
    }
}

impl<'r> Responder<'r, 'static> for HTTPException {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> response::Result<'static> {
        let error_response = serde_json::to_string(
            &(ErrorResponse {
                message: self.to_string(),
            }),
        )
        .unwrap();

        Response::build()
            .status(self.get_http_status())
            .header(ContentType::JSON)
            .sized_body(error_response.len(), Cursor::new(error_response))
            .ok()
    }
}

impl<'r> Responder<'r, 'static> for HTTPSuccessResponse {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> response::Result<'static> {
        let custom_response = self.get_http_response();

        let response = serde_json::to_string(
            &(CustomResponse {
                message: custom_response.message,
                data: custom_response.data,
            }),
        )
        .unwrap();

        Response::build()
            .status(self.get_http_status())
            .header(ContentType::JSON)
            .sized_body(response.len(), Cursor::new(response))
            .ok()
    }
}
