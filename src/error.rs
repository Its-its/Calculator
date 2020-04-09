use std::fmt;

use crate::ExprToken;

pub type Result<I> = std::result::Result<I, Error>;

#[derive(Debug, Clone)]
pub enum Error {
	InputEmpty,
	Text(String),
	UnexpectedToken(ExprToken)
}


impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Error::Text(e) => write!(f, "{:?}", e),
			Error::UnexpectedToken(e) => write!(f, "UnexpectedToken: {:?}", e),
			Error::InputEmpty => write!(f, "InputEmpty")
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