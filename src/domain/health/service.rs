use crate::infrastructure::db::connection::check_connection;

pub async fn check_system_health() -> Result<String, String> {
    match check_connection().await {
        Ok(ok) => {
            return Ok(ok);
        }
        Err(err) => {
            return Err(err);
        }
    };
}
