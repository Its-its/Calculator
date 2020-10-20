use std::fmt;

use conversion::Error as ConversionError;
use crate::{ExprToken, Operator};

pub type Result<I> = std::result::Result<I, Error>;

#[derive(Debug, Clone)]
pub enum Error {
	ExpectedArgument,
	ExpectedQuantity,
	InvalidFunction,
	InvalidOperator,
	InputEmpty,

	UnexpectedToken(ExprToken),
	Conversion(ConversionError),
	UnableToOperateValues(Operator),
	UnableToConvertValues(String, String)
}


impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Error::UnableToConvertValues(a, b) => write!(f, r#"Values of type "{}" and "{}" are not able to be compaired or converted."#, a, b),
			Error::UnableToOperateValues(o) => write!(f, "Unable to operate: {}", o),
			Error::UnexpectedToken(e) => write!(f, "Unexpected Token: {}", e),
			Error::InputEmpty => write!(f, "Input Empty"),
			Error::ExpectedArgument => write!(f, "Expected Argument"),
			Error::ExpectedQuantity => write!(f, "Expected Quantity"),
			Error::InvalidFunction => write!(f, "Invalid Function"),
			Error::InvalidOperator => write!(f, "Invalid Operator"),
			Error::Conversion(e) => e.fmt(f)
		}
	}
}


impl From<ConversionError> for Error {
	fn from(value: ConversionError) -> Self {
		Error::Conversion(value)
	}
}