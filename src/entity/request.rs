use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ethereum_types::{U256, Address};

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct Request {
    pub id: i32,
    pub model_id: i32,
    pub from_address: Address,
    pub prompt: String,
    pub request_data: Option<Vec<u8>>,
    pub fee_amount: U256,
    pub request_status: RequestStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(strum_macros::Display)]
pub enum RequestStatus {
    Fulfilled,
    Pending,
    Failed,
}
