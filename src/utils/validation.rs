//! A trait that defines validation methods for a payment request.
//!
//! This trait is used to ensure that the `PaymentRequest` object contains
//! valid data before processing. It provides methods to validate:
//! - The payment amount must be greater than zero.
//! - The currency code must be a valid 3-letter ISO 4217 string.
//! - The customer ID, if provided, must not be empty.
use crate::error::PaymentError;
use crate::models::PaymentRequest;

/// This trait is used to ensure that the `PaymentRequest` object contains
/// valid data before processing.
pub trait ValidatableRequest {
    /// Validates the entire payment request, including amount, currency,
    /// and customer ID.
    ///
    /// # Errors
    /// Returns `PaymentError::ValidationError` if any validation fails.
    fn validate(&self) -> Result<(), PaymentError>;

    /// Validates that the payment amount is greater than zero.
    ///
    /// # Errors
    /// Returns `PaymentError::ValidationError` if the amount is zero or negative.
    fn validate_amount(&self) -> Result<(), PaymentError>;

    /// Validates that the currency code is a 3-character ISO 4217 string.
    ///
    /// # Errors
    /// Returns `PaymentError::ValidationError` if the currency code is invalid.
    fn validate_currency(&self) -> Result<(), PaymentError>;

    /// Validates that the customer ID, if provided, is not an empty string.
    ///
    /// # Errors
    /// Returns `PaymentError::ValidationError` if the customer ID is empty.
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
        if self.amount == 0 {
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
