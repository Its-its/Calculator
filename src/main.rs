pub static DEBUG_MODE: bool = true;


#[macro_use]
macro_rules! print_dbg {
	() => (if crate::DEBUG_MODE { println!(); });
	($($arg:tt)*) => (if crate::DEBUG_MODE { println!($($arg)*); });
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


// TODO: Ability to calc hd video 2.1 million pixels/frame * 25 frams/second * 24 bits/pixel = 1.1 gigabits/second

fn main() -> Result<()> {
	let mut factory  = Factory::new();

	println!("Got: {}", factory.parse(r#"200 - 100 - 10%"#)?);

	Ok(())
}



#[cfg(test)]
mod tests {
	use super::*;


	macro_rules! test {
		($factory:expr, $eval:expr, $result:expr) => {
			assert_eq!($factory.parse($eval).unwrap(), Value::new_quantity($result));
		};
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