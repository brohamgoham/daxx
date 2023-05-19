//! Daxx API response types
//!
//! This module contains the response types for Daxx API calls
//! and the implementation of the `IntoResponse` trait for them.
// allow dead code


#[allow(dead_code)]
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
/// Response for Daxx API calls
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DaxxSuccessResponse<T: Serialize> {
    data: T,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct DaxxErrorResponse {
    msg: Option<String>,

    #[serde(rename = "code")]
    status: u16,
}

impl<T: Serialize> DaxxSuccessResponse<T>
where
    T: Serialize,
{
    pub(crate) fn _send(data: T) -> Self {
        DaxxSuccessResponse { data }
    }
}

impl DaxxErrorResponse {
    pub(crate) fn _send(status: u16, msg: Option<String>) -> Self {
        DaxxErrorResponse { status, msg }
    }
}

impl IntoResponse for DaxxErrorResponse {
    fn into_response(self) -> Response {
        (
            StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            Json(self),
        )
            .into_response()
    }
}
