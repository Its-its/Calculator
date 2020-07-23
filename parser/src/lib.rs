#![warn(warnings, rust_2018_idioms, unsafe_code, dead_code)]
#![warn(clippy::all)]
#![allow(clippy::missing_docs_in_private_items, unsafe_code)]

pub static mut DEBUG_MODE: bool = false;


#[macro_use]
macro_rules! print_dbg {
	() => (if crate::is_debug() { println!(); });
	($($arg:tt)*) => (if crate::is_debug() { println!($($arg)*); });
}

pub fn enable_debug() {
	unsafe { DEBUG_MODE = true; }
}

pub fn is_debug() -> bool {
	unsafe { DEBUG_MODE }
}

pub mod factory;
pub mod tokenizer;
pub mod tokens;
pub mod parser;
pub mod error;
pub mod value;
pub mod operations;
pub mod units;
pub mod functions;
pub mod consts;
pub mod equations;


pub use factory::Factory;
pub use error::{Error, Result};
pub use tokens::{Operator, ExprToken, TokenType};
pub use parser::{Parser, ParseValue};
pub use tokenizer::Tokenizer;
pub use value::Value;
pub use operations::ExpressionArg;




#[cfg(test)]
mod tests {
	use super::*;


	macro_rules! test {
		($factory:expr, $eval:expr, $result:expr) => {
			assert_eq!($factory.parse($eval).unwrap(), Value::new_quantity($result));
		};
	}

	#[test]
	fn test_basics() {
		let factory  = Factory::new();

		test!(factory, "1 + 1", 2.0);
		test!(factory, "1 - 1", 0.0);
		test!(factory, "2 * 2", 4.0);
		test!(factory, "10 / 2", 5.0);
		test!(factory, "2^2", 4.0);
		test!(factory, "2^2^2", 16.0);

		test!(factory, "1 + (1 + 1)", 3.0);
		test!(factory, "(1 - 1) + 1", 1.0);
		test!(factory, "1 + (2 * 5)", 11.0);
		test!(factory, "(2 * 5) / 5", 2.0);
	}

	#[test]
	fn test_operations() {
		let factory  = Factory::new();

		test!(factory, "3GB - 1GB", 2.0);
		test!(factory, "1GB + 1GB", 2.0);
		test!(factory, "1GB * 1GB", 1.0);
		test!(factory, "4GB / 2GB", 2.0);
		test!(factory, "1GB == 1GB", 1.0);
		test!(factory, "2GB > 1GB", 1.0);
		test!(factory, "2GB >= 1GB", 1.0);
		test!(factory, "1GB < 2GB", 1.0);
		test!(factory, "1GB <= 2GB", 1.0);
	}

	#[test]
	fn test_functions() {
		let factory  = Factory::new();

		test!(factory, "max(1.5, 10.0)", 10.0);
		test!(factory, "max(1.5, 10.0, 30.0, 15.0)", 30.0);
	}
}