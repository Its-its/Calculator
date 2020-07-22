use std::fmt;

pub type Result<I> = std::result::Result<I, Error>;

#[derive(Debug, Clone)]
pub enum Error {
	ExpectedArgument,
	Text(String)
}


impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Error::Text(e) => write!(f, "{:?}", e),
			Error::ExpectedArgument => write!(f, "Expected Argument")
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