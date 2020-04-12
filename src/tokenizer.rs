use regex::Regex;

use crate::{ExprToken, Operator, Result};
use crate::consts::{default_constants, DefaultConsts};

pub type Id<T> = (&'static str, T);

pub static DOUBLE_CHAR_TOKENS: [Id<ExprToken>; 6] = [
	("->", ExprToken::Operator(Operator::ConvertInto)),
	("<=", ExprToken::Operator(Operator::LessThanOrEqual)),
	(">=", ExprToken::Operator(Operator::GreaterThanOrEqual)),
	("!=", ExprToken::Operator(Operator::DoesNotEqual)),
	("~=", ExprToken::Operator(Operator::ApproxEqual)),
	("==", ExprToken::Operator(Operator::DoubleEqual))
];

pub static SINGLE_CHAR_TOKENS: [Id<ExprToken>; 16] = [
	(",", ExprToken::Comma),
	("(", ExprToken::StartGrouping),
	(")", ExprToken::EndGrouping),
	("[", ExprToken::StartGrouping),
	("]", ExprToken::EndGrouping),
	("{", ExprToken::StartGrouping),
	("}", ExprToken::EndGrouping),
	("+", ExprToken::Operator(Operator::Plus)),
	("-", ExprToken::Operator(Operator::Minus)),
	("=", ExprToken::Operator(Operator::Equal)),
	("<", ExprToken::Operator(Operator::LessThan)),
	(">", ExprToken::Operator(Operator::GreaterThan)),
	("*", ExprToken::Operator(Operator::Multiply)),
	("/", ExprToken::Operator(Operator::Divide)),
	("%", ExprToken::Operator(Operator::Division)),
	("^", ExprToken::Operator(Operator::Caret))
];


pub type ParseResult = Option<ExprToken>;


pub struct Tokenizer<'a> {
	consts: DefaultConsts<'a>,
	value: &'a str,
	pos: usize
}

impl<'a> Tokenizer<'a> {
	pub fn new(value: &'a str) -> Self {
		Tokenizer {
			value,
			pos: 0,
			consts: default_constants()
		}
	}

	pub fn parse(&mut self) -> Result<Vec<ExprToken>> {
		let mut compiled = Vec::new();

		while !self.is_finished() {
			let found = None
				.or_else(|| self.remove_non_essiential())
				.or_else(|| self.parse_tokens(&DOUBLE_CHAR_TOKENS))
				.or_else(|| self.parse_tokens(&SINGLE_CHAR_TOKENS))
				.or_else(|| self.parse_number())
				.or_else(|| self.parse_literal());

			if let Some(found) = found {
				compiled.push(found);
			} else {
				eprintln!("Unable to finish. Stopped at: {:?}", self.value.get(self.pos..).unwrap_or(""));

				break;
			}
		}

		Ok(compiled.into_iter().filter(|i| i != &ExprToken::Whitespace).collect())
	}

	fn remove_non_essiential(&mut self) -> ParseResult {
		if self.consume_if(" ") {
			Some(ExprToken::Whitespace)
		} else {
			None
		}
	}

	fn parse_number(&mut self) -> ParseResult {
		let remains = self.get_remaining_str();

		let mut builder = Regex::new(r#"^((?:[0-9,]+)?\.?(?:e-?)?(?:[0-9]+)?)"#).unwrap();

		if let Some(found) = builder.find(remains) {
			let mut end = found.end();

			if end != 0 {
				let num = {
					// If ending has a comma remove it.
					if remains.get(end - 1..end) == Some(",") {
						end -= 1;
					}

					let number = remains.get(0..end).unwrap();

					number.replace(",", "").parse().unwrap()
				};

				self.consume_amount(end);

				Some(ExprToken::Number(num))
			} else {
				None
			}
		} else {
			None
		}
	}

	fn parse_literal(&mut self) -> ParseResult {
		let remains = self.get_remaining_str();

		let mut builder = Regex::new(r#"^([a-zA-Z]+)"#).unwrap();

		if let Some(found) = builder.find(remains) {
			if found.end() != 0 {
				let value = remains.get(0..found.end()).unwrap().to_owned();

				self.consume_amount(found.end());

				// Check if it's a const.
				if let Some(item) = self.consts.iter().find(|i| i.0 == value.as_str()) {
					Some(ExprToken::Number(item.1))
				} else {
					Some(ExprToken::Literal(value))
				}
			} else {
				None
			}
		} else {
			None
		}
	}

	fn parse_tokens(&mut self, tokens: &[Id<ExprToken>]) -> ParseResult {
		for token in tokens {
			if self.consume_if(token.0) {
				return Some(token.1.clone());
			}
		}

		None
	}

	//

	fn get_remaining_str(&self) -> &str {
		self.value.get(self.pos..)
		.unwrap_or("")
	}

	fn consume_if(&mut self, next_token: &str) -> bool {
		if let Some(grabbed) = self.value.get(self.pos..self.pos + next_token.len()) {
			if grabbed == next_token {
				self.consume_amount(next_token.len());
				true
			} else {
				false
			}
		} else {
			false
		}
	}

	fn consume_amount(&mut self, value: usize) {
		self.pos += value;
	}

	fn is_finished(&self) -> bool {
		self.value.len() <= self.pos
	}
}