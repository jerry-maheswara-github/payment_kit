//! 
//! It abstracts over operations such as creating payments, checking their status, and processing refunds.

use crate::error::PaymentError;
use crate::models::{PaymentRequest, PaymentResponse, RefundResponse};
use crate::status::PaymentStatus;

/// Trait for implementing payment gateway integrations.
pub trait PaymentGateway {
    /// Create a new payment based on the request payload.
    fn create_payment(&self, req: PaymentRequest) -> Result<PaymentResponse, PaymentError>;

    /// Check the status of a transaction by its ID.
    fn check_status(&self, transaction_id: &str) -> Result<PaymentStatus, PaymentError>;

    /// Initiate a refund for a given transaction ID.
    fn refund(&self, transaction_id: &str) -> Result<RefundResponse, PaymentError>;
}
