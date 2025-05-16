# 💳 payment_kit

A powerful and extensible payment toolkit for Rust — featuring core data models, robust error handling,
flexible status flows, and seamless integration into any payment system.

Designed to be embedded in services that communicate with third-party payment providers,
the `payment_kit` ensures consistency and testability across environments.

---

## ✨ Features

- 🔌 Pluggable `PaymentGateway` trait for easy integration with third-party providers
- 🧾 Strongly-typed `PaymentRequest` and `PaymentResponse` structures
- ✅ Input validation via the `ValidatableRequest` trait as an option
- 🛑 Rich, structured error types with `PaymentError`
- 🧪 Built-in mock gateway for testing and development
- 💳 Support for various payment instruments (e.g., credit cards, e-wallets, bank transfers).

---

## 🚀 Quick Start

```rust
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
        eprintln!("Validation failed: {}", e);
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
```

---

## 📄 License

Licensed under the [Apache-2.0 license](http://www.apache.org/licenses/LICENSE-2.0.txt)

---

## 👨 Author

Jerry Maheswara <jerrymaheswara@gmail.com>

---

## ❤️ Built with Love in Rust

This project is built with ❤️ using **Rust** — a systems programming language that is safe, fast, and concurrent.  
Rust is the perfect choice for building reliable and efficient applications.

---

## 🤝 Contributing

Pull requests, issues, and feedback are welcome!  
If you find this crate useful, give it a ⭐ and share it with others in the Rustacean community.

---
