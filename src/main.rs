// https://en.wikipedia.org/wiki/List_of_equations

pub mod tokenizer;
pub mod tokens;
pub mod parser;
pub mod error;
pub mod value;
pub mod equations;
pub mod units;
pub mod functions;
pub mod consts;

pub use error::{Error, Result};
pub use tokens::{Operator, ExprToken};
pub use parser::Parser;
pub use tokenizer::Tokenizer;
pub use value::Value;


fn main() -> Result<()> {
    println!("Got: {}", Parser::new("max(1.5, 10.0)").parse()?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operations() {
        assert_eq!(Parser::new("3GB - 1GB").parse().unwrap(), Value::new_quantity(2.0));
        assert_eq!(Parser::new("1GB + 1GB").parse().unwrap(), Value::new_quantity(2.0));
        assert_eq!(Parser::new("1GB * 1GB").parse().unwrap(), Value::new_quantity(1.0));
        assert_eq!(Parser::new("4GB / 2GB").parse().unwrap(), Value::new_quantity(2.0));
        assert_eq!(Parser::new("1GB == 1GB").parse().unwrap(), Value::new_quantity(1.0));
        assert_eq!(Parser::new("2GB > 1GB").parse().unwrap(), Value::new_quantity(1.0));
        assert_eq!(Parser::new("2GB >= 1GB").parse().unwrap(), Value::new_quantity(1.0));
        assert_eq!(Parser::new("1GB < 2GB").parse().unwrap(), Value::new_quantity(1.0));
        assert_eq!(Parser::new("1GB <= 2GB").parse().unwrap(), Value::new_quantity(1.0));
    }
}