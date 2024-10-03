use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
pub struct BugReportRequest {
    pub id: i32,
    pub mail_address: String,
    pub subject: String,
    pub mail_body: String,
    pub detail: String,
    pub screenshot: Vec<u8>,
}
