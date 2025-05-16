use payment_kit::models::{PaymentInstrument, PaymentRequest, PaymentResponse, RefundResponse};
use payment_kit::processor::PaymentProcessor;
use payment_kit::utils::validation::ValidatableRequest;
use payment_kit::error::PaymentError;
use payment_kit::gateway::PaymentGateway;
use payment_kit::status::PaymentStatus;

pub struct MockPaymentGateway;

impl PaymentGateway for MockPaymentGateway {

    fn create_payment(&self, req: PaymentRequest) -> Result<PaymentResponse, PaymentError> {
        if req.amount == 0 || req.order_id.contains("fail") {
            return Err(PaymentError::InvalidRequest("Simulated failure".into()));
        }

        Ok(PaymentResponse {
            transaction_id: format!("mock_txn_{}", req.order_id),
            amount: req.amount,
            payment_instrument: req.payment_instrument,
            status: PaymentStatus::Pending,
            redirect_url: Some("https://mock.payment/redirect".to_string()),
        })
    }
    fn check_status(&self, transaction_id: &str) -> Result<PaymentStatus, PaymentError> {
        if transaction_id.contains("fail") {
            Err(PaymentError::ProcessingError("Transaction failed".into()))
        } else {
            Ok(PaymentStatus::Success)
        }
    }

    fn refund(&self, transaction_id: &str) -> Result<RefundResponse, PaymentError> {
        if transaction_id.contains("notfound") {
            Err(PaymentError::InvalidRequest("Transaction ID not found".into()))
        } else {
            Ok(RefundResponse {
                refund_id: format!("refund_{}", transaction_id),
                status: PaymentStatus::Refunded,
                transaction_id: transaction_id.to_string(),
                refunded: true,
            })
        }
    }
}

fn main() -> Result<(), PaymentError> {
    let gateway = MockPaymentGateway;
    let processor = PaymentProcessor::new(&gateway);

    let request = PaymentRequest {
        order_id: "INV-001".to_string(),
        amount: 100_000,
        currency: "IDR".to_string(),
        payment_instrument: PaymentInstrument::EWallet {
            provider: "OVO".to_string(),
        },
        customer_id: Some("cust-123".to_string()),
        description: Some("Order payment".to_string()),
        metadata: None,
    };

    if let Err(e) = request.validate() {
        eprintln!("Validation failed: {}", e.to_string());
    }

    let response = processor.create_payment(request);
    match response {
        Ok(res) => {
            println!("Payment successful: {:#?}", res);
        }
        Err(err) => {
            eprintln!("Payment failed: {}", err);
        }
    }

    Ok(())
}