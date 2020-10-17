use std::ops::Range;

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

pub type RangedType = Range<usize>;
pub type ParseResult = Option<(RangedType, ExprToken)>;


pub struct Tokenizer<'a> {
	pub factory: &'a Factory,
	compiled: Vec<(RangedType, ExprToken)>,

	value: StringSlice<'a>
}

impl<'a> Tokenizer<'a> {
	pub fn new(value: &'a str, factory: &'a Factory) -> Self {
		Tokenizer {
			factory,
			value: StringSlice::new(value),
			compiled: Vec::new(),
		}
	}

	pub fn get_tokens(&self) -> Vec<ExprToken> {
		self.compiled.iter().map(|(_, e)| e.clone()).collect()
	}

	pub fn get_compiled(&self) -> &[(RangedType, ExprToken)] {
		self.compiled.as_ref()
	}

	pub fn parse(&mut self) -> Result<()> {
		if self.value.is_finished() {
			return Ok(());
		}

		while !self.value.is_finished() {
			let found = None
				.or_else(|| self.remove_non_essiential())
				.or_else(|| self.parse_tokens(&DOUBLE_CHAR_TOKENS))
				.or_else(|| self.parse_tokens(&SINGLE_CHAR_TOKENS))
				.or_else(|| self.parse_number())
				.or_else(|| self.parse_literal());

			if let Some(found) = found {
				self.compiled.push(found);
			} else {
				print_dbg!("Unable to finish. Stopped at: {:?}", self.value.get_remaining_str());

				break;
			}
		}

		Ok(())
	}


	fn remove_non_essiential(&mut self) -> ParseResult {
		if self.value.consume_if(" ") {
			Some((self.value.pos - 1..self.value.pos, ExprToken::Whitespace))
		} else {
			None
		}
	}

	fn parse_number(&mut self) -> ParseResult {
		let start_pos = self.value.pos;
		let remains = self.value.get_remaining_str();

		let builder = Regex::new(r#"^((?:[0-9,]+)?\.?(?:e-?)?(?:[0-9]+)?)"#).unwrap();

		if let Some(found) = builder.find(remains) {
			let mut end = found.end();

			if end != 0 {
				let num  = {
					// If ending has a comma remove it.
					if remains.get(end - 1..end) == Some(",") {
						end -= 1;
					}

					let number = remains.get(0..end).unwrap();

					number.replace(",", "").parse().unwrap()
				};

				self.value.consume_amount(end);

				Some((start_pos..start_pos + end, ExprToken::Number(num)))
			} else {
				None
			}
		} else {
			None
		}
	}

	fn parse_literal(&mut self) -> ParseResult {
		let start_pos = self.value.pos;
		let remains = self.value.get_remaining_str();

		let builder = Regex::new(r#"^([^\d\s\(\)\[\]\{\}]+)"#).unwrap();

		if let Some(found) = builder.find(remains) {
			let end = found.end();

			if end != 0 {
				let found = found.as_str().to_string();

				self.value.consume_amount(end);

				// Check if it's a const.
				if let Some(item) = self.factory.find_const(found.as_str()) {
					Some((start_pos..start_pos + end, ExprToken::Number(item)))
				} else {
					Some((start_pos..start_pos + end, ExprToken::Literal(found)))
				}
			} else {
				None
			}
		} else {
			None
		}
	}

	fn parse_tokens(&mut self, tokens: &[Id<ExprToken>]) -> ParseResult {
		let start_pos = self.value.pos;

		for token in tokens {
			if self.value.consume_if(&token.0) {
				return Some((start_pos..self.value.pos, token.1.clone()));
			}
		}

		None
	}

	pub fn find_tokens<'b>(&'b self, tokens: &'b [TokenType]) -> impl Iterator<Item = &[(RangedType, ExprToken)]> + 'b {
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

			if tokens[state.step] == token.1 {
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


struct StringSlice<'a> {
	value: &'a str,
	pos: usize
}


impl<'a> StringSlice<'a> {
	pub fn new(value: &'a str) -> StringSlice<'a> {
		StringSlice {
			value,
			pos: 0
		}
	}

	fn consume_if(&mut self, next_token: &str) -> bool {
		if let Some(grabbed) = self.get_until(next_token.len()) {
			let is_correct = grabbed == next_token;

			if is_correct {
				self.consume_amount(next_token.len());
			}

			is_correct
		} else {
			false
		}
	}

	pub fn consume_amount(&mut self, value: usize) {
		self.pos += value;
	}

	pub fn is_finished(&self) -> bool {
		self.value.len() <= self.pos
	}

	pub fn get_remaining_str(&self) -> &str {
		self.value.get(self.pos..)
		.unwrap_or("")
	}

	pub fn get_until(&self, value: usize) -> Option<&str> {
		self.value.get(self.pos..self.pos + value)
	}
}