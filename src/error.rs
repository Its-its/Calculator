use std::fmt;

use conversion::Error as ConversionError;
use crate::ExprToken;

pub type Result<I> = std::result::Result<I, Error>;

#[derive(Debug, Clone)]
pub enum Error {
	ExpectedArgument,
	InputEmpty,
	Text(String),
	UnexpectedToken(ExprToken),
	Conversion(ConversionError)
}


impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Error::Text(e) => write!(f, "{:?}", e),
			Error::UnexpectedToken(e) => write!(f, "Unexpected Token: {:?}", e),
			Error::InputEmpty => write!(f, "Input Empty"),
			Error::ExpectedArgument => write!(f, "Expected Argument"),
			Error::Conversion(e) => e.fmt(f)
		}
	}
}


impl From<String> for Error {
	fn from(value: String) -> Self {
		Error::Text(value)
	}
}

impl From<&str> for Error {
	fn from(value: &str) -> Self {
		Error::Text(value.to_string())
	}
}

impl From<ConversionError> for Error {
	fn from(value: ConversionError) -> Self {
		Error::Conversion(value)
	}
}