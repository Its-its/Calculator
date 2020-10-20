use std::fmt;

use conversion_parser::Error as ConversionParserError;

pub type Result<I> = std::result::Result<I, Error>;

#[derive(Debug, Clone)]
pub enum Error {
	ConversionParser(ConversionParserError)
}


impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Error::ConversionParser(e) => e.fmt(f)
		}
	}
}


impl From<ConversionParserError> for Error {
	fn from(value: ConversionParserError) -> Self {
		Error::ConversionParser(value)
	}
}