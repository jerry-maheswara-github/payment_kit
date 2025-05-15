use crate::error::PaymentError;
use crate::models::{PaymentRequest, PaymentResponse, RefundResponse};
use crate::status::PaymentStatus;

pub trait PaymentGateway {
    fn create_payment(&self, req: PaymentRequest) -> Result<PaymentResponse, PaymentError>;
    fn check_status(&self, transaction_id: &str) -> Result<PaymentStatus, PaymentError>;
    fn refund(&self, transaction_id: &str) -> Result<RefundResponse, PaymentError>;
}