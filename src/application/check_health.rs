use crate::domain::health::service::check_system_health;
use crate::dto::system_health_dto::SystemHealthDto;
use chrono::Utc;

pub async fn check_health() -> Result<SystemHealthDto, SystemHealthDto> {
    let version = env!("CARGO_PKG_VERSION").to_string();
    let timestamp = Utc::now().to_string();

    match check_system_health().await {
        Ok(ok) => {
            return Ok(SystemHealthDto {
                message: ok,
                version,
                timestamp,
            });
        }
        Err(err) => {
            return Err(SystemHealthDto {
                message: err,
                version,
                timestamp,
            });
        }
    };
}
