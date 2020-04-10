use std::ops::RangeBounds;

use conversion::{Quantity, BaseUnit};

use crate::{Operator, ExprToken, Tokenizer, Result, Error, Value};
use crate::equations::{Add, Subtract, Divide, Multiply, Literal, Grouping, ExpressionArg};

//

pub type ExpressionResult<'a> = Result<Option<ExpressionArg>>;


macro_rules! return_value {
	($parser:expr, ExprToken::$token:ident) => {{
		match $parser.next().ok_or(Error::InputEmpty)? {
			ExprToken::$token(v) => v,
			t @ _ => return Err(Error::UnexpectedToken(t))
		}
	}};
}


pub struct Parser<'a> {
	tokenizer: Tokenizer<'a>,
	eval: &'a str
}

impl<'a> Parser<'a> {
	pub fn new(eval: &'a str) -> Self {
		Parser {
			tokenizer: Tokenizer::new(eval),
			eval
		}
	}

	pub fn parse(&mut self) -> Result<()> {
		let tokens = self.tokenizer.parse()?;

		println!("Parsed Tokens: {:?}", tokens);
		println!("");

		let mut slicer = TokenSlicer::new(tokens);

		while true {
			let to_parse = self.parse_tokens(&mut slicer)?;

			println!("Tokens: {:?}", slicer.tokens);
			println!("");

			if let Some(to_parse) = to_parse {
				println!("Calculated: {}", to_parse.eval()?);
				break;
			}
		}


		Ok(())
	}

	pub fn parse_tokens(&self, slicer: &mut TokenSlicer) -> ExpressionResult {
		// EXPONENTS ^
		let found_exp = slicer.find(&Operator::Caret.into());
		if let Some(pos) = found_exp.first() {
			return self.parse_exponents(*pos, slicer);
		}

		// GROUPINGS ( [ {  } ] )
		let found_grps = slicer.find(&ExprToken::StartGrouping);
		if let Some(pos) = found_grps.first() {
			return self.parse_parentheses(*pos, slicer);
		}


		// Should be Multiple OR Divide. Whichever comes first.
		let mut found_md = slicer.find(&Operator::Multiply.into());
		found_md.append(&mut slicer.find(&Operator::Divide.into()));
		found_md.sort();

		if let Some(pos) = found_md.first() {
			return self.parse_operation(*pos, slicer);
		}

		// Should be Add OR Subtract. Whichever comes first.
		let mut found_pm = slicer.find(&Operator::Plus.into());
		found_pm.append(&mut slicer.find(&Operator::Minus.into()));
		found_pm.sort();

		if let Some(pos) = found_pm.first() {
			return self.parse_operation(*pos, slicer);
		}

		// Should be Conversion.
		let mut found_ci = slicer.find(&Operator::ConvertInto.into());
		if let Some(pos) = found_ci.first() {
			return self.parse_operation(*pos, slicer);
		}


		self.parse_finished(slicer)
	}

	pub fn parse_exponents(&self, pos: usize, slicer: &mut TokenSlicer) -> ExpressionResult {
		slicer.backward();
		slicer.set_pos(pos - 1);
		let base = self.parse_number_expression(slicer)?;
		let start_pos = slicer.get_pos();

		slicer.forward();
		slicer.set_pos(pos + 1);
		let power = self.parse_number_expression(slicer)?;
		let end_pos = slicer.get_pos();

		slicer.reset_pos();

		let expr = Operator::Caret.compare(base.ok_or(Error::InputEmpty)?, power.ok_or(Error::InputEmpty)?);

		slicer.replace(start_pos..end_pos, expr.eval()?.into_tokens());

		Ok(None)
	}

	pub fn parse_parentheses(&self, start_pos: usize, slicer: &mut TokenSlicer) -> ExpressionResult {
		// Find end, create new slicer inside of grouping.
		slicer.set_pos(start_pos + 1);
		slicer.forward();

		loop {
			if let Some(item) = slicer.next() {
				if item == ExprToken::StartGrouping {
					return self.parse_parentheses(slicer.get_pos() - 1, slicer);
				}

				if item == ExprToken::EndGrouping {
					let end_pos = slicer.get_pos();
					// slicer without the start and end Groupings
					let mut group_slicer = slicer.clone_from(start_pos + 1, end_pos - 1);

					self.parse_tokens(&mut group_slicer)?;

					if group_slicer.tokens.len() == 1 {
						slicer.replace(start_pos..end_pos, group_slicer.tokens);
					} else {
						slicer.replace(start_pos + 1..end_pos - 1, group_slicer.tokens);
					}

					return Ok(None)
				}
			} else {
				return Err(Error::InputEmpty);
			}
		}

		Ok(None)
	}

	pub fn parse_operation(&self, pos: usize, slicer: &mut TokenSlicer) -> ExpressionResult {
		let operator = slicer.get(pos).cloned().unwrap().into_operator();

		slicer.backward();
		slicer.set_pos(pos - 1);
		let prev = self.parse_number_expression(slicer)?;
		let start_pos = slicer.get_pos();

		slicer.forward();
		slicer.set_pos(pos + 1);
		let next = self.parse_number_expression(slicer)?;
		let end_pos = slicer.get_pos();

		slicer.reset_pos();

		let expr = operator.compare(prev.ok_or(Error::InputEmpty)?, next.ok_or(Error::InputEmpty)?);

		slicer.replace(start_pos..end_pos, expr.eval()?.into_tokens());

		Ok(None)
	}


	fn parse_number_expression<'b>(&self, slicer: &mut TokenSlicer) -> ExpressionResult<'b> {
		if slicer.is_reversed() {
			let unit = self.parse_unit_expression(slicer)?;

			if slicer.is_next_value_func(|v| v.is_number()) {
				let value = return_value!(slicer, ExprToken::Number);

				return Ok(Some(Box::new(Literal::new(Value::Quantity(Quantity::new_unit(value, unit))))));
			}

			else if let Some(unit) = unit {
				return Ok(Some(Box::new(Literal::new(Value::Unit(unit)))));
			}
		} else {
			if slicer.is_next_value_func(|v| v.is_number()) {
				let value = return_value!(slicer, ExprToken::Number);
				let unit = self.parse_unit_expression(slicer)?;

				return Ok(Some(Box::new(Literal::new(Value::Quantity(Quantity::new_unit(value, unit))))));
			}

			else if let Some(unit) = self.parse_unit_expression(slicer)? {
				return Ok(Some(Box::new(Literal::new(Value::Unit(unit)))));
			}
		}

		Ok(None)
	}

	fn parse_unit_expression(&self, slicer: &mut TokenSlicer) -> Result<Option<Box<dyn BaseUnit>>> {
		if slicer.is_next_value_func(|v| v.is_literal()) {
			let val = return_value!(slicer, ExprToken::Literal);

			crate::units::get_unit_from_literal(&val)
			.map(|i| Some(i))
			.ok_or(format!("No known unit named \"{}\"", val).into())
		} else {
			Ok(None)
		}
	}


	fn parse_finished(&self, slicer: &mut TokenSlicer) -> ExpressionResult {
		slicer.forward();
		slicer.reset_pos();

		match self.parse_number_expression(slicer)? {
			Some(i) => {
				if slicer.is_finished() {
					slicer.clear();
					Ok(Some(i))
				} else {
					Err("Unable to parse remaining tokens.".into())
				}
			},
			None => Err("Unable to parse current tokens.".into())
		}
	}
}


pub struct TokenSlicer {
	reversed: bool,
	tokens: Vec<ExprToken>,
	pos: usize
}

impl TokenSlicer {
	pub fn new(tokens: Vec<ExprToken>) -> Self {
		TokenSlicer {
			tokens,
			pos: 0,
			reversed: false
		}
	}

	pub fn is_finished(&self) -> bool {
		self.get_pos() == self.tokens.len()
	}

	pub fn clear(&mut self) {
		self.tokens.clear();
	}

	pub fn is_reversed(&mut self) -> bool {
		self.reversed
	}

	pub fn forward(&mut self) {
		self.reversed = false;
	}

	pub fn backward(&mut self) {
		self.reversed = true;
	}

	pub fn get_pos(&self) -> usize {
		self.pos
	}

	pub fn set_pos(&mut self, value: usize) {
		self.pos = value;
	}

	pub fn reset_pos(&mut self) {
		self.pos = 0;
	}

	pub fn consume(&mut self, amount: usize) {
		if self.reversed {
			if self.pos != 0 {
				self.pos -= amount;
			}
		} else {
			self.pos += amount;
		}
	}

	pub fn find(&self, token: &ExprToken) -> Vec<usize> {
		self.tokens.iter()
			.enumerate()
			.filter(|(_, e)| e == &token)
			.map(|(u, _)| u)
			.collect()
	}

	pub fn find_fn<F>(&self, exp_fn: F) -> Vec<usize> where F: Fn(&ExprToken) -> bool {
		self.tokens.iter()
			.enumerate()
			.filter(|(_, e)| exp_fn(e))
			.map(|(u, _)| u)
			.collect()
	}

	pub fn clone_from(&self, start: usize, end: usize) -> TokenSlicer {
		TokenSlicer {
			pos: 0,
			reversed: false,
			tokens: self.tokens.get(start..end).map(|i| i.to_vec()).unwrap()
		}
	}

	pub fn replace<I>(&mut self, range: I, replace_with: Vec<ExprToken>) where I: RangeBounds<usize> {
		let _ = self.tokens.splice(range, replace_with).count();
	}

	pub fn next(&mut self) -> Option<ExprToken> {
		let token = self.tokens.get(self.pos).cloned();

		self.consume(1);

		token
	}

	pub fn peek(&self) -> Option<&ExprToken> {
		self.get(self.pos)
	}

	pub fn get(&self, index: usize) -> Option<&ExprToken> {
		self.tokens.get(index)
	}

	pub fn is_next_value(&self, value: &ExprToken) -> bool {
		self.tokens.get(self.pos)
			.map(|f| f == value)
			.unwrap_or_default()
	}

	pub fn is_next_value_func<F: FnOnce(&ExprToken) -> bool>(&self, func: F) -> bool {
		match self.peek() {
			Some(t) => func(t),
			None => false
		}
	}

	pub fn consume_if_next(&mut self, value: &ExprToken) -> bool {
		if let Some(found) = self.tokens.get(self.pos) {
			if found == value {
				self.consume(1);
				return true;
			}
		}

		false
	}

	pub fn consume_token(&mut self, value: &ExprToken) -> Result<()> {
		if let Some(found) = self.tokens.get(self.pos) {
			if found == value {
				self.consume(1);
				return Ok(());
			}
		}

		Err("Not the next token.".into())
	}
}