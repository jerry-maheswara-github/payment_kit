use serde::{Deserialize, Serialize};

/// Enum representing the current state of a payment.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PaymentStatus {
    Pending,
    Success,
    Failed,
    Cancelled,
    Refunded,
}
