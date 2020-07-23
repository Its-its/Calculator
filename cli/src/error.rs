use std::fmt;

use conversion_parser::Error as ParserError;

pub type Result<T> = std::result::Result<T, Error>;


#[derive(Debug)]
pub enum Error {
	ParserError(ParserError)
}

impl From<ParserError> for Error {
	fn from(err: ParserError) -> Self {
		Self::ParserError(err)
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("Error")
		.finish()
	}
}