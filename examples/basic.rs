use payment_kit::error::PaymentError;
use payment_kit::gateway::PaymentGateway;
use payment_kit::models::{CallbackPayload, CallbackValidationResult, PaymentRequest, PaymentResponse, RefundRequest, RefundResponse};

#[tokio::main]
async fn main() {
    let payment_request = PaymentRequest {
        amount: 100.0,
        currency: "USD".to_string(),
        customer_id: "customer_123".to_string(),
        description: Some("Test payment".to_string()),
    };

    let gateway = DummyGateway;

    match gateway.initiate_payment(payment_request).await {
        Ok(response) => {
            println!("Payment initiated successfully: {:?}", response);
        }
        Err(e) => {
            eprintln!("Error initiating payment: {:?}", e);
        }
    }
}

struct DummyGateway;

#[async_trait::async_trait]
impl PaymentGateway for DummyGateway {
    async fn initiate_payment(&self, _req: PaymentRequest) -> Result<PaymentResponse, PaymentError> {
        Ok(PaymentResponse {
            transaction_id: "txn_123456".to_string(),
            redirect_url: Some("https://example.com/redirect".to_string()),
            status: "success".to_string(),
        })
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
