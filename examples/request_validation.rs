use payment_kit::models::PaymentRequest;
use payment_kit::error::PaymentError;
use payment_kit::models::PaymentInstrument;
use payment_kit::utils::validation::ValidatableRequest;

fn main() {
    let request: PaymentRequest = PaymentRequest {
        order_id: "".to_string(),
        amount: 0,
        currency: "IDR".into(),
        customer_id: None,
        description: None,
        metadata: None,
        payment_instrument: PaymentInstrument::CreditCard { provider: "Visa".to_string() },
    };

    match request.validate_amount() {
        Ok(_) => println!("validate_amount: Request is valid."),
        Err(e) => match e {
            PaymentError::ValidationError(msg) => eprintln!("Validation failed: {}", msg),
            PaymentError::Unknown(_e) => eprintln!("An unknown error occurred."),
            _ => eprintln!("Unexpected error: {}", e),
        },
    }

    match request.validate_currency() {
        Ok(_) => println!("validate_currency: Request is valid."),
        Err(e) => match e {
            PaymentError::ValidationError(msg) => eprintln!("Validation failed: {}", msg),
            PaymentError::Unknown(_e) => eprintln!("An unknown error occurred."),
            _ => eprintln!("Unexpected error: {}", e),
        },
    }
    
    match request.validate_customer_id() {
        Ok(_) => println!("validate_customer_id: Request is valid."),
        Err(e) => match e {
            PaymentError::ValidationError(msg) => eprintln!("Validation failed: {}", msg),
            PaymentError::Unknown(_e) => eprintln!("An unknown error occurred."),
            _ => eprintln!("Unexpected error: {}", e),
        },
    }
}
