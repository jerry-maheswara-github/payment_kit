use crate::error::PaymentError;
use crate::models::PaymentRequest;

pub trait ValidatableRequest {
    fn validate(&self) -> Result<(), PaymentError>;

    fn validate_amount(&self) -> Result<(), PaymentError>;
    fn validate_currency(&self) -> Result<(), PaymentError>;
    fn validate_customer_id(&self) -> Result<(), PaymentError>;
}

impl ValidatableRequest for PaymentRequest {
    fn validate(&self) -> Result<(), PaymentError> {
        self.validate_amount()?;
        self.validate_currency()?;
        self.validate_customer_id()?;
        Ok(())
    }

    fn validate_amount(&self) -> Result<(), PaymentError> {
        if self.amount <= 0.0 {
            return Err(PaymentError::ValidationError("Amount must be greater than zero.".into()));
        }
        Ok(())
    }

    fn validate_currency(&self) -> Result<(), PaymentError> {
        if self.currency.trim().len() != 3 {
            return Err(PaymentError::ValidationError("Currency must be a valid 3-letter ISO code.".into()));
        }
        Ok(())
    }

    fn validate_customer_id(&self) -> Result<(), PaymentError> {
        if let Some(id) = &self.customer_id {
            if id.trim().is_empty() {
                return Err(PaymentError::ValidationError("Customer ID, if provided, cannot be empty.".into()));
            }
        }
        Ok(())
    }

}
