// https://en.wikipedia.org/wiki/List_of_equations

pub mod factory;
pub mod tokenizer;
pub mod tokens;
pub mod parser;
pub mod error;
pub mod value;
pub mod equations;
pub mod units;
pub mod functions;
pub mod consts;

pub use factory::Factory;
pub use error::{Error, Result};
pub use tokens::{Operator, ExprToken};
pub use parser::Parser;
pub use tokenizer::Tokenizer;
pub use value::Value;


// TODO: Ability to calc hd video 2.1 million pixels/frame * 25 frams/second * 24 bits/pixel = 1.1 gigabits/second

fn main() -> Result<()> {
	let mut factory  = Factory::new();

	println!("Got: {}", factory.parse("1000 MB/s -> GB/min")?);

	Ok(())
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_operations() {
		let factory  = Factory::new();

		assert_eq!(factory.parse("3GB - 1GB").unwrap(), Value::new_quantity(2.0));
		assert_eq!(factory.parse("1GB + 1GB").unwrap(), Value::new_quantity(2.0));
		assert_eq!(factory.parse("1GB * 1GB").unwrap(), Value::new_quantity(1.0));
		assert_eq!(factory.parse("4GB / 2GB").unwrap(), Value::new_quantity(2.0));
		assert_eq!(factory.parse("1GB == 1GB").unwrap(), Value::new_quantity(1.0));
		assert_eq!(factory.parse("2GB > 1GB").unwrap(), Value::new_quantity(1.0));
		assert_eq!(factory.parse("2GB >= 1GB").unwrap(), Value::new_quantity(1.0));
		assert_eq!(factory.parse("1GB < 2GB").unwrap(), Value::new_quantity(1.0));
		assert_eq!(factory.parse("1GB <= 2GB").unwrap(), Value::new_quantity(1.0));
	}

	#[test]
	fn test_functions() {
		let factory  = Factory::new();

		assert_eq!(factory.parse("max(1.5, 10.0)").unwrap(), Value::new_quantity(10.0));
		assert_eq!(factory.parse("max(1.5, 10.0, 30.0, 15.0)").unwrap(), Value::new_quantity(30.0));
	}
}