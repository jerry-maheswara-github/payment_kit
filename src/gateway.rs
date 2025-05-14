use async_trait::async_trait;

use crate::error::PaymentError;
use crate::models::{CallbackPayload, CallbackValidationResult, PaymentRequest, PaymentResponse, RefundRequest, RefundResponse};

#[async_trait]
pub trait PaymentGateway: Send + Sync {
    async fn initiate_payment(&self, req: PaymentRequest) -> Result<PaymentResponse, PaymentError>;

    async fn validate_callback(&self, payload: CallbackPayload) -> Result<CallbackValidationResult, PaymentError>;

    async fn refund(&self, req: RefundRequest) -> Result<RefundResponse, PaymentError>;
}
