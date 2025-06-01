use payment_kit::error::PaymentError;
use payment_kit::gateway::PaymentGateway;
use payment_kit::models::{PaymentRequest, PaymentResponse, RefundResponse};
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

#[cfg(test)]
mod tests {
    use payment_kit::models::{PaymentInstrument, PaymentRequest};
    use payment_kit::processor::PaymentProcessor;
    use crate::MockPaymentGateway;
    use super::*;

    #[test]
    fn test_create_payment_success() {
        let gateway = MockPaymentGateway;
        let processor = PaymentProcessor::new(&gateway);

        let req = PaymentRequest {
            order_id: "ORDER123".into(),
            amount: 100_000,
            currency: "IDR".into(),
            payment_instrument: PaymentInstrument::EWallet {
                provider: "MockPay".into(),
            },
            customer_id: Some("CUST001".into()),
            description: Some("Test payment".into()),
            metadata: None,
        };

        let result = processor.create_payment(req);
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.amount, 100_000);
        assert_eq!(response.status, PaymentStatus::Pending);
    }
    
    #[test]
    fn test_create_payment_failed() {
        let gateway = MockPaymentGateway;
        let processor = PaymentProcessor::new(&gateway);

        let req = PaymentRequest {
            order_id: "ORDER_FAIL".into(),
            amount: 0, // trigger failure
            currency: "IDR".into(),
            payment_instrument: PaymentInstrument::EWallet {
                provider: "MockPay".into(),
            },
            customer_id: Some("CUST001".into()),
            description: Some("Invalid test payment".into()),
            metadata: None,
        };

        let result = processor.create_payment(req);
        assert!(result.is_err());

        match result {
            Err(PaymentError::InvalidRequest(msg)) => {
                assert_eq!(msg, "Simulated failure");
            }
            _ => panic!("Expected InvalidRequest error"),
        }
    }

}
