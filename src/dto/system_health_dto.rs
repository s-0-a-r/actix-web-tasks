use serde::Serialize;

#[derive(Serialize)]
pub struct SystemHealthDto {
    pub message: String,
    pub version: String,
    pub timestamp: String,
}
