use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentStatus {
    Pending,
    Paid,
    Failed,
    Expired,
    Cancelled,
    Refunded,
    Success,
    Authorized,
    Captured,
    Chargeback,
    Declined,
    Void,
    PendingSettlement,
}