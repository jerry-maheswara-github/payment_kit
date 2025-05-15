use payment_kit::models::PaymentRequest;
use payment_kit::error::PaymentError;
use payment_kit::utils::validation::ValidatableRequest;

fn main() {
    let request: PaymentRequest = PaymentRequest {
        amount: 0.0,
        currency: "IDR".into(),
        customer_id: None,
        description: None,
        metadata: None,
    };

    match request.validate_customer_id() {
        Ok(_) => println!("Request is valid."),
        Err(e) => match e {
            PaymentError::ValidationError(msg) => eprintln!("Validation failed: {}", msg),
            PaymentError::Unknown => eprintln!("An unknown error occurred."),
            _ => eprintln!("Unexpected error: {}", e),
        },
    }
}
