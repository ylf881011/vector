use serde::Serialize;

/// HTTP success response structure used by sources returning JSON bodies.
#[allow(missing_docs)]
#[derive(Serialize, Debug)]
pub struct SuccessResponse {
    pub data: serde_json::Value,
    pub code: u16,
    pub status: u16,
    pub msg: String,
    pub error: bool,
}
#[allow(missing_docs)]
#[derive(Serialize, Debug)]
pub struct ErrorResponse {
    pub data: serde_json::Value,
    pub code: u16,
    pub status: u16,
    pub msg: String,
    pub error: bool,
}