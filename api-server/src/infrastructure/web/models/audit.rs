use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuditContext {
    pub user_id: i32,
    pub ip: String,
    pub user_agent: String,
}
