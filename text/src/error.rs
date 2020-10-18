use std::fmt;

use conversion_parser::Error as ConversionParserError;

pub type Result<I> = std::result::Result<I, Error>;

#[derive(Debug, Clone)]
pub enum Error {
	Text(String),
	ConversionParser(ConversionParserError)
}


impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Error::Text(e) => write!(f, "{:?}", e),
			Error::ConversionParser(e) => e.fmt(f)
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

impl From<ConversionParserError> for Error {
	fn from(value: ConversionParserError) -> Self {
		Error::ConversionParser(value)
	}
}