pub static DEBUG_MODE: bool = false;


#[macro_use]
macro_rules! print_dbg {
	() => ();
	($($arg:tt)*) => ();
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