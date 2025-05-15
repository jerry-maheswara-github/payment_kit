use serde::{Deserialize, Serialize};
use crate::status::{PaymentStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentInstrument {
    CreditCard { provider: String },
    VirtualAccount { provider: String },
    BankTransfer { provider: String },
    EWallet { provider: String },
    Custom { provider: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentRequest {
    pub order_id: String,
    pub amount: u64,
    pub currency: String,
    pub payment_instrument: PaymentInstrument,
    pub customer_id: Option<String>,
    pub description: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentResponse {
    pub transaction_id: String,
    pub amount: u64,
    pub payment_instrument: PaymentInstrument,
    pub status: PaymentStatus,
    pub redirect_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundRequest {
    pub transaction_id: String,
    pub amount: u64,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundResponse {
    pub refund_id: String,
    pub status: PaymentStatus,
    pub transaction_id: String,
    pub refunded: bool,
}
