use std::fmt;

pub type Result<I> = std::result::Result<I, Error>;

#[derive(Debug, Clone)]
pub enum Error {
	ExpectedArgument,
	MissingUnit
}


impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Error::ExpectedArgument => write!(f, "Expected Argument"),
			Error::MissingUnit => write!(f, "Missing Unit")
		}
	}
}