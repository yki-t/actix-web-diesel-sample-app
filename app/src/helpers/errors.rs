#![allow(clippy::all)]

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use anyhow;
use serde_json::json;
use std::fmt;

/// エラー定義
#[derive(Debug)]
pub struct Error {
    pub err: anyhow::Error,
    pub status: u16,
}

/// To compartibale with actix-web 1
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.err)
    }
}

/// To compartibale with actix-web 2
impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        let err_json = json!({ "error": self.err.to_string() });
        HttpResponse::build(StatusCode::from_u16(self.status).unwrap()).json(err_json)
    }
}

/// To compartibale with anyhow
impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Error {
        Error { err, status: 500 }
    }
}

/// To compartibale with diesel
impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Error {
        Error {
            err: err.into(),
            status: 500,
        }
    }
}
