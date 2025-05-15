use serde::{Deserialize, Serialize};
use crate::status::{PaymentStatus, TransactionStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentRequest {
    pub amount: f64,
    pub currency: String,
    pub customer_id: Option<String>,
    pub description: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentResponse {
    pub transaction_id: String,
    pub redirect_url: Option<String>,
    pub status: PaymentStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundRequest {
    pub transaction_id: String,
    pub amount: f64,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundResponse {
    pub refund_id: String,
    pub status: PaymentStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallbackPayload {
    pub transaction_id: String,
    pub status: PaymentStatus,
    pub raw_payload: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallbackValidationResult {
    pub valid: bool,
    pub status: PaymentStatus,
    pub transaction_id: String,
}

pub struct PaymentDetail {
    pub transaction_id: String,
    pub payment_status: PaymentStatus,
    pub transaction_status: Option<TransactionStatus>,
    pub updated_at: String,
}
