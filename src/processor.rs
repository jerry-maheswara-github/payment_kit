use crate::error::PaymentError;
use crate::gateway::PaymentGateway;  
use crate::models::{PaymentRequest, PaymentResponse, RefundResponse};
use crate::status::PaymentStatus;

pub struct PaymentProcessor<'a> {
    gateway: &'a dyn PaymentGateway,  
}

impl<'a> PaymentProcessor<'a> {
    pub fn new(gateway: &'a dyn PaymentGateway) -> Self {
        PaymentProcessor { gateway }
    }

    pub fn create_payment(&self, req: PaymentRequest) -> Result<PaymentResponse, PaymentError> {
        self.gateway.create_payment(req)
    }

    pub fn check_status(&self, transaction_id: &str) -> Result<PaymentStatus, PaymentError> {
        self.gateway.check_status(transaction_id)
    }

    pub fn refund(&self, transaction_id: &str) -> Result<RefundResponse, PaymentError> {
        self.gateway.refund(transaction_id)
    }
}
