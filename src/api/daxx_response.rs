use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};

/// Response for Daxx API calls
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DaxxSuccessResponse<T: Serialize> {
    data: T
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct DaxxErrorResponse {
    msg: Option<String>,

    #[serde(rename = "code")]
    status: u16
}

impl <T: Serialize> DaxxResponse<T> 
where 
    T: Serialize
    {
        pub(crate) fn send(data: T) -> Self {
            DaxxSuccessResponse { data }
        }
    }

impl DaxxErrorResponse {
    pub(crate) fn send(status: u16, msg: Option<String>) -> Self {
        DaxxErrorResponse { status, msg}
    }
}