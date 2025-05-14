use payment_kit::error::PaymentError;
use payment_kit::gateway::PaymentGateway;
use payment_kit::models::{CallbackPayload, CallbackValidationResult, PaymentRequest, PaymentResponse, RefundRequest, RefundResponse};

#[tokio::test]
async fn test_initiate_payment_success() {
    let payment_request = PaymentRequest {
        amount: 200.0,
        currency: "USD".to_string(),
        customer_id: "customer_456".to_string(),
        description: Some("Test payment".to_string()),
    };

    let gateway = MockGateway;

    let result = gateway.initiate_payment(payment_request).await;

    assert!(result.is_ok());
    let payment_response = result.unwrap();
    assert_eq!(payment_response.status, "success");
    assert_eq!(payment_response.transaction_id, "txn_123456");
}

#[tokio::test]
async fn test_validate_callback_success() {
    let callback_payload = CallbackPayload {
        transaction_id: "txn_123456".to_string(),
        status: "completed".to_string(),
        raw_payload: "dummy_payload".to_string(),
    };

    let gateway = MockGateway;

    let result = gateway.validate_callback(callback_payload).await;

    assert!(result.is_ok());
    let validation_result = result.unwrap();
    assert!(validation_result.valid);
    assert_eq!(validation_result.status, "completed");
}

struct MockGateway;

#[async_trait::async_trait]
impl PaymentGateway for MockGateway {
    async fn initiate_payment(&self, req: PaymentRequest) -> Result<PaymentResponse, PaymentError> {
        if req.amount > 0.0 {
            Ok(PaymentResponse {
                transaction_id: "txn_123456".to_string(),
                redirect_url: Some("https://example.com/redirect".to_string()),
                status: "success".to_string(),
            })
        } else {
            Err(PaymentError::InvalidRequest("Invalid amount".to_string()))
        }
    }

    async fn validate_callback(&self, _payload: CallbackPayload) -> Result<CallbackValidationResult, PaymentError> {
        Ok(CallbackValidationResult {
            valid: true,
            status: "completed".to_string(),
        })
    }

    async fn refund(&self, _req: RefundRequest) -> Result<RefundResponse, PaymentError> {
        Ok(RefundResponse {
            refund_id: "refund_123".to_string(),
            status: "refunded".to_string(),
        })
    }
}
