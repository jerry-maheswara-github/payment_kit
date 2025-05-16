//!
//! It provides high-level methods to create payments, check transaction statuses, and process refunds.

use crate::error::PaymentError;
use crate::gateway::PaymentGateway;
use crate::models::{PaymentRequest, PaymentResponse, RefundResponse};
use crate::status::PaymentStatus;

/// A processor that delegates payment operations to a configured `PaymentGateway`.
pub struct PaymentProcessor<'a> {
    gateway: &'a dyn PaymentGateway,
}

impl<'a> PaymentProcessor<'a> {
    /// Create a new `PaymentProcessor` with the given gateway.
    pub fn new(gateway: &'a dyn PaymentGateway) -> Self {
        PaymentProcessor { gateway }
    }

    /// Initiate a payment through the configured gateway.
    pub fn create_payment(&self, req: PaymentRequest) -> Result<PaymentResponse, PaymentError> {
        self.gateway.create_payment(req)
    }

    /// Check the status of a transaction by its ID.
    pub fn check_status(&self, transaction_id: &str) -> Result<PaymentStatus, PaymentError> {
        self.gateway.check_status(transaction_id)
    }

    /// Request a refund for a given transaction.
    pub fn refund(&self, transaction_id: &str) -> Result<RefundResponse, PaymentError> {
        self.gateway.refund(transaction_id)
    }
}
