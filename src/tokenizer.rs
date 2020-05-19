use regex::Regex;

use crate::{ExprToken, TokenType, Operator, Result, Factory};

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
	pub factory: &'a Factory,
	compiled: Vec<ExprToken>,

	value: &'a str,
	pos: usize
}

impl<'a> Tokenizer<'a> {
	pub fn new(value: &'a str, factory: &'a Factory) -> Self {
		Tokenizer {
			factory,
			value,
			pos: 0,
			compiled: Vec::new(),
		}
	}

	pub fn get_compiled(&self) -> &[ExprToken] {
		self.compiled.as_ref()
	}

	pub fn parse(&mut self) -> Result<()> {
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

		self.compiled = compiled.into_iter().filter(|i| i != &ExprToken::Whitespace).collect();

		Ok(())
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

		let mut builder = Regex::new(r#"^([^\d\s]+)"#).unwrap();

		if let Some(found) = builder.find(remains) {
			if found.end() != 0 {
				let value = remains.get(0..found.end()).unwrap().to_owned();

				self.consume_amount(found.end());

				// Check if it's a const.
				if let Some(item) = self.factory.find_const(value.as_str()) {
					Some(ExprToken::Number(item))
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

	pub fn is_finished(&self) -> bool {
		self.value.len() <= self.pos
	}


	pub fn find_tokens<'b>(&'b self, tokens: &'b [TokenType]) -> impl Iterator<Item = &[ExprToken]> + 'b {
		self.find_tokens_index(tokens)
			.map(move |(start, end)| self.compiled.get(start..=end).unwrap())
	}

	/// Will find matching tokens starting index.
	pub fn find_tokens_index<'b>(&'b self, tokens: &'b [TokenType]) -> impl Iterator<Item = (usize, usize)> + 'b {
		let initial_state = Found {
			start: 0,
			end: 0,
			step: 0,
			pass: false
		};

		self.compiled.iter()
		.enumerate()
		.scan(initial_state, move |state, (pos, token)| {
			if state.pass || state.step == 0 {
				state.pass = false;
				state.start = pos;
				state.end = 0;
				state.step = 0;
			}

			if &tokens[state.step] == token {
				state.step += 1;
			} else {
				state.step = 0;
			}

			if tokens.len() == state.step {
				state.pass = true;
				state.end = pos;
			}

			Some((state.pass, state.start, state.end))
		})
		.filter(|s| s.0)
		.map(|s| (s.1, s.2))
	}
}

struct Found {
	start: usize, // Start pos
	end: usize, // Finished pos.
	step: usize, // Current token step.
	pass: bool // Should pass filter.
}