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
    Parser::new("1m + 0.5").parse()?;

    Ok(())
}