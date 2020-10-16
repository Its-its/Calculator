use std::fmt;
use std::ops::RangeBounds;

use rust_decimal::Decimal;

use conversion::{Quantity, Units};

use crate::{Factory, Operator, ExprToken, Tokenizer, Result, Error, Value};
use crate::operations::{Literal, Function, ExpressionArg};

#[derive(Debug)]
pub struct Expression {
	pub args: ExpressionArg,
	pub range: Option<(usize, usize)>
}

impl Expression {
	pub fn new(args: ExpressionArg) -> Self {
		Self {
			args,
			range: None
		}
	}

	pub fn new_range(args: ExpressionArg, range: (usize, usize)) -> Self {
		Self {
			args,
			range: Some(range)
		}
	}
}

impl From<ExpressionArg> for Expression {
	fn from(args: ExpressionArg) -> Self {
		Self {
			args,
			range: None
		}
	}
}

pub type ExpressionResult = Result<Option<Expression>>;


macro_rules! return_value {
	($parser:expr, ExprToken::$token:ident) => {{
		match $parser.next().ok_or(Error::InputEmpty)? {
			ExprToken::$token(v) => v,
			t => return Err(Error::UnexpectedToken(t))
		}
	}};
}


#[derive(Debug)]
pub enum ParseValue {
	Single(Value),
	Multi(Vec<ExprToken>)
}

impl ParseValue {
	pub fn into_tokens(self) -> Vec<ExprToken> {
		match self {
			ParseValue::Single(v) => v.into_tokens(),
			ParseValue::Multi(v) => v
		}
	}
}

impl fmt::Display for ParseValue {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			ParseValue::Single(v) => v.fmt(f),
			ParseValue::Multi(v) => {
				for token in v {
					token.fmt(f)?;
				}

				Ok(())
			}
		}
	}
}

impl PartialEq<Value> for ParseValue {
	fn eq(&self, other: &Value) -> bool {
		match self {
			ParseValue::Single(v) => v == other,
			_ => false
		}
	}
}


pub struct Parser<'a> {
	factory: &'a Factory,
	tokenizer: Tokenizer<'a>,
	pub steps: Vec<Vec<ExprToken>>
}

impl<'a> Parser<'a> {
	pub fn new(factory: &'a Factory, eval: &'a str) -> Self {
		Parser {
			factory,
			steps: Vec::new(),
			tokenizer: Tokenizer::new(eval, factory),
		}
	}

	pub fn new_with_tokenizer(factory: &'a Factory, tokenizer: Tokenizer<'a>) -> Self {
		Parser {
			factory,
			tokenizer,
			steps: Vec::new(),
		}
	}

	pub fn get_parsed_tokens(&self) -> &[ExprToken] {
		self.tokenizer.get_compiled()
	}

	pub fn parse(&mut self) -> Result<ParseValue> {
		self.tokenizer.parse()?;

		print_dbg!("Parsed Tokens: {:?}", self.get_parsed_tokens());

		let mut slicer = TokenSlicer::new(self.get_parsed_tokens().to_vec());

		if self.parse_neighbors(&mut slicer)? {
			self.steps.push(slicer.tokens.clone());
		}

		loop {
			slicer.reset_pos();
			slicer.forward();

			let current_operation = self.parse_tokens(&mut slicer)?;

			// This should be what it JUST did.
			match current_operation {
				Some(to_parse) => {
					if !slicer.is_finished() {
						if let Some(replace_range) = to_parse.range {
							let eval = to_parse.args.eval()?;
							slicer.replace(replace_range.0..replace_range.1, eval.into_tokens());
						}

						if !slicer.tokens.is_empty() {
							self.steps.push(slicer.tokens.clone());
						}

						print_dbg!("");
					} else {
						print_dbg!("");

						print_dbg!("Steps:");
						print_dbg!(" - {:?}", self.get_parsed_tokens().iter().map(|t| format!("{}", t)).collect::<Vec<String>>().join(" "));
						for step in self.steps.as_slice() {
							print_dbg!(" - {:?}", step.iter().map(|t| format!("{}", t)).collect::<Vec<String>>().join(" "));
						}

						return Ok(ParseValue::Single(to_parse.args.eval()?));
					}
				}

				None => {
					print_dbg!("");
					print_dbg!("Unable to continue parsing...");
					print_dbg!("{:?}", slicer.tokens);

					print_dbg!("Steps:");
					print_dbg!(" - {:?}", self.get_parsed_tokens().iter().map(|t| format!("{}", t)).collect::<Vec<String>>().join(" "));
					for step in self.steps.as_slice() {
						print_dbg!(" - {:?}", step.iter().map(|t| format!("{}", t)).collect::<Vec<String>>().join(" "));
					}

					return Ok(ParseValue::Multi(slicer.tokens));
				}
			}
		}
	}

	fn parse_neighbors(&self, slicer: &mut TokenSlicer) -> Result<bool> {
		let mut updated = false;
		// [ Number(5.0), Literal("min"), Number(30.0), Literal("sec") ]
		// [ Number(5.5), Literal("min") ]

		while let Some(token) = slicer.peek() {
			// Number, Literal
			if token.is_literal() && slicer.peek_previous().map(|p| p.is_number()).unwrap_or_default() {
				slicer.prev_pos();

				let start_pos = slicer.get_pos();

				let mut neighbors = Vec::new();

				// TODO: Ensure they have the same Base Literal.
				// Will run into issues down the road otherwise.

				while let Some(expr) = self.parse_number_expression(slicer)? {
					neighbors.push(expr.args);
				}

				if neighbors.len() < 2 {
					continue;
				}

				updated = true;

				let cmp = neighbors.into_iter()
				.fold::<ExpressionArg, _>(Box::new(Literal::new(Value::Quantity(Quantity::new(Decimal::new(0, 0))))), |a, b| {
					Operator::Plus.compare(a, b)
				});

				slicer.replace(start_pos..slicer.get_pos(), cmp.eval()?.into_tokens());

				break;
			}

			slicer.next_pos();
		}


		Ok(updated)
	}

	pub fn parse_tokens(&self, slicer: &mut TokenSlicer) -> ExpressionResult {
		print_dbg!(" - Parse: {:?}", slicer.tokens);

		// EXPONENTS ^
		let found_exp = slicer.find_indexes(&Operator::Caret.into());
		if let Some(pos) = found_exp.first() {
			return self.parse_exponents(*pos, slicer);
		}

		// GROUPINGS ( [ {  } ] )
		let found_grps = slicer.find_multiple_indexes(&[ExprToken::StartGrouping, ExprToken::EndGrouping]).into_iter();
		for pos in found_grps {
			let found = self.parse_parentheses(pos, slicer)?;
			if found.is_some() { return Ok(found); }
		}


		let mut found_ops = slicer.find_multiple_indexes(&[Operator::Multiply.into(), Operator::Divide.into()]);
		found_ops.append(&mut slicer.find_multiple_indexes(&[Operator::Plus.into(), Operator::Minus.into()]));
		found_ops.append(&mut slicer.find_indexes(&Operator::ConvertInto.into()));
		found_ops.append(&mut slicer.find_multiple_indexes(&[
			Operator::GreaterThan.into(),
			Operator::GreaterThanOrEqual.into(),
			Operator::LessThan.into(),
			Operator::LessThanOrEqual.into(),
			Operator::DoubleEqual.into(),
			Operator::DoesNotEqual.into(),
		]));

		let found_ops = found_ops.into_iter();

		for pos in found_ops {
			let found = self.parse_operation(pos, slicer)?;
			if found.is_some() { return Ok(found); }
		}

		self.parse_finished(slicer)
	}

	pub fn parse_exponents(&self, pos: usize, slicer: &mut TokenSlicer) -> ExpressionResult {
		print_dbg!("parse_exponents");

		let pos_from_end = slicer.tokens.len() - pos;

		// Work backward to get what's before the expontent.
		slicer.backward();
		slicer.set_pos(pos);
		slicer.next_pos();


		let paren = slicer.peek().unwrap() == &ExprToken::EndGrouping;

		let base = if paren {
			self.parse_parentheses(slicer.get_pos(), slicer)?
		} else {
			self.parse_number_expression(slicer)?
		};

		// Return if range is defined in parentheses.
		if paren && base.as_ref().map(|e| e.range.is_some()).unwrap_or_default() {
			return Ok(base);
		}

		let start_pos = slicer.get_pos();

		slicer.forward();
		slicer.set_pos(slicer.tokens.len() - pos_from_end + 1);

		let power = self.parse_number_expression(slicer)?;
		let end_pos = slicer.get_pos();

		slicer.reset_pos();

		let expr = Operator::Caret.compare(
			base.ok_or(Error::InputEmpty)?.args,
			power.ok_or(Error::InputEmpty)?.args
		);

		Ok(Some(Expression::new_range(expr, (start_pos, end_pos))))
	}

	pub fn parse_parentheses(&self, mut start_pos: usize, slicer: &mut TokenSlicer) -> ExpressionResult {
		print_dbg!("parse_parentheses: {} - {}", start_pos, slicer.is_reversed());

		slicer.set_pos(start_pos);
		// slicer.next_pos();

		// If is reversed find the StartGrouping and continue from there.
		if slicer.is_reversed() {
			let mut nest_depth = 0;

			loop {
				let next_expr = slicer.next().ok_or(Error::InputEmpty)?;

				if next_expr == ExprToken::EndGrouping {
					nest_depth += 1;
				} else if next_expr == ExprToken::StartGrouping {
					if nest_depth != 0 {
						nest_depth -= 1;
					} else {
						break;
					}
				}
			}


			let actual_start = std::cmp::min(start_pos, slicer.get_pos());

			slicer.forward();
			// Add 1 to Ignore Start Grouping.
			slicer.set_pos(actual_start + 1);
			start_pos = actual_start;
		}

		// If it was reversed, slicer pos should now be set to start of parentheses.

		if start_pos != 0 {
			// Is it a literal before the parentheses?
			// If so it's a function.
			if let Some(ExprToken::Literal(func_name)) = slicer.peek_previous() {
				print_dbg!(" - Function Literal: {}", func_name);

				let func = self.factory.find_func(func_name)
					.ok_or_else(|| Error::Text("Not a valid function.".into()))?;

				// Capture everything after Function Name and inside the parentheses.
				let mut inner_slicer = slicer.clone_from(start_pos + 1, slicer.tokens.len() - 1);

				let mut params = Vec::new();

				while let Some(expr) = self.parse_number_expression(&mut inner_slicer)? {
					params.push(expr.args);

					if !inner_slicer.consume_if_next(&ExprToken::Comma) {
						break;
					}
				}

				let end_pos = slicer.pos + inner_slicer.pos + 2;

				return Ok(Some(Expression::new_range(
					Box::new(Function::new(func, params)),
					(start_pos - 1, end_pos)
				)));
			}
		}

		let mut insides = Vec::new();

		// Skip start Grouping
		if slicer.peek().map(|e| e == &ExprToken::StartGrouping).unwrap_or_default() {
			slicer.next_pos();
		}

		loop {
			if let Some(item) = slicer.next() {
				// Inner grouping?
				if item == ExprToken::StartGrouping {
					if crate::is_debug() {
						print_dbg!(" - Inner: {}", insides.iter().map(|t| format!("{}", t)).collect::<Vec<String>>().join(" "));
					}

					return self.parse_parentheses(slicer.get_pos() - 1, slicer);
				}

				if item == ExprToken::EndGrouping {
					if crate::is_debug() {
						print_dbg!(" - Inner: {}", insides.iter().map(|t| format!("{}", t)).collect::<Vec<String>>().join(" "));
					}

					let end_pos = slicer.get_pos();

					// The insides of the grouping only.
					let mut group_slicer = slicer.clone_from(start_pos + 1, end_pos - 1);

					let parsed = self.parse_tokens(&mut group_slicer)?;

					slicer.reset_pos();

					if let Some(mut expr) = parsed {
						if let Some(range) = expr.range.as_mut() {
							// Check if start and end of range is at the start and end of group.
							// If so, extend range in both directions by one to remove groupings too.
							let i_s = start_pos + range.0 + 1;
							let i_e = start_pos + range.1 + 1;

							if start_pos + 1 == i_s && end_pos - 1 == i_e {
								*range = (i_s - 1, i_e + 1);
							} else {
								*range = (i_s, i_e);
							}
						}

						return Ok(Some(expr));
					} else {
						return Ok(None);
					}

				}

				if crate::is_debug() {
					insides.push(item.clone());
				}
			} else {
				return Err(Error::InputEmpty);
			}
		}
	}

	pub fn parse_operation(&self, pos: usize, slicer: &mut TokenSlicer) -> ExpressionResult {
		print_dbg!("parse_operation");

		let operator = slicer.get(pos).cloned().unwrap().into_operator();

		slicer.backward();
		slicer.set_pos(pos - 1);

		let prev = self.parse_number_expression(slicer)?;

		slicer.forward();
		slicer.set_pos(pos + 1);
		let next = self.parse_number_expression(slicer)?;
		let end_pos = slicer.get_pos();

		slicer.reset_pos();


		let prev = prev.ok_or(Error::InputEmpty)?;
		let next = next.ok_or(Error::InputEmpty)?;


		// Ensure they have same unit
		if !crate::units::can_operate(&prev.args, &next.args) {
			return Ok(None);
		}


		let start_pos = prev.range.unwrap().0;

		let expr = operator.compare(
			prev.args,
			next.args
		);

		Ok(Some(Expression::new_range(expr, (start_pos, end_pos))))
	}


	fn parse_number_expression(&self, slicer: &mut TokenSlicer) -> ExpressionResult {
		let start_pos = slicer.get_pos();

		if slicer.is_reversed() {
			let unit = self.parse_unit_expression(slicer)?;

			if slicer.is_next_value_func(|v| v.is_number()) {
				// Temp fix.
				let pos = slicer.get_pos();

				let value = return_value!(slicer, ExprToken::Number);

				return Ok(Some(
					Expression::new_range(
						Box::new(Literal::new(Value::Quantity(Quantity::new_unit(value, unit)))),
						(pos, start_pos.max(slicer.get_pos()))
					)
				));
			}

			else if let Some(unit) = unit {
				return Ok(Some(
					Expression::new_range(
						Box::new(Literal::new(Value::Unit(unit))),
						(start_pos.min(slicer.get_pos()), start_pos.max(slicer.get_pos()))
					)
				));
			}
		} else if slicer.is_next_value_func(|v| v.is_number()) {
			let value = return_value!(slicer, ExprToken::Number);
			let mut unit = self.parse_unit_expression(slicer)?;

			if unit.is_none() {
				// Account for Percentage:
				//  - 10 - 10%
				//  - [Number(10), Minus, Number(10), Division]
				//  - 9
				// Change Division into a Unit. Check for unit when operating.
				// Checks for Division. Then checks to see if token after it is a operator or doesn't exist.
				if slicer.is_next_value(&Operator::Division.into()) && slicer.get(slicer.get_pos() + 1).map(|t| t.is_operator()).unwrap_or(true) {
					slicer.next_pos();
					unit = Some(Units::new(self.factory.find_unit("%")));
				}
			}

			return Ok(Some(
				Expression::new_range(
					Box::new(Literal::new(Value::Quantity(Quantity::new_unit(value, unit)))),
					(start_pos.min(slicer.get_pos()), start_pos.max(slicer.get_pos()))
				)
			));
		} else if let Some(unit) = self.parse_unit_expression(slicer)? {
			return Ok(Some(
				Expression::new_range(
					Box::new(Literal::new(Value::Unit(unit))),
					(start_pos.min(slicer.get_pos()), start_pos.max(slicer.get_pos()))
				)
			));
		}

		Ok(None)
	}

	fn parse_unit_expression(&self, slicer: &mut TokenSlicer) -> Result<Option<Units>> {
		if slicer.is_next_value_func(|v| v.is_literal()) {
			let literal_val = return_value!(slicer, ExprToken::Literal);

			let mut units = Vec::new();
			let split = literal_val.split('/');

			for name in split {
				let base_unit = self.factory.find_unit(name);

				units.push(base_unit);
			}

			Ok(Some(Units::new_vec(units)))
		} else {
			Ok(None)
		}
	}


	fn parse_finished(&self, slicer: &mut TokenSlicer) -> ExpressionResult {
		print_dbg!("parse_finished");

		slicer.forward();
		slicer.reset_pos();

		match self.parse_number_expression(slicer)? {
			Some(i) => {
				if slicer.is_finished() || slicer.tokens.len() == 1 {
					slicer.clear();
					Ok(Some(i))
				} else {
					slicer.reset_pos();
					Ok(None)
				}
			},
			None => Ok(None)
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
		self.get_pos() == self.tokens.len() || self.tokens.is_empty()
	}

	pub fn clear(&mut self) {
		self.tokens.clear();
	}

	pub fn is_reversed(&self) -> bool {
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

	pub fn next_pos(&mut self) {
		if self.is_reversed() {
			self.pos -= 1;
		} else {
			self.pos += 1;
		}
	}

	pub fn prev_pos(&mut self) {
		if self.is_reversed() {
			self.pos += 1;
		} else {
			self.pos -= 1;
		}
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

	pub fn find_indexes(&self, token: &ExprToken) -> Vec<usize> {
		let mut found: Vec<usize> = self.tokens.iter()
			.enumerate()
			.filter(|(i, e)| {
				if e != &token {
					false
				} else if self.is_reversed() {
					i <= &self.get_pos()
				} else {
					i >= &self.get_pos()
				}
			})
			.map(|(u, _)| u)
			.collect();

		if self.is_reversed() {
			found.reverse();
		}

		found
	}

	pub fn find_multiple_indexes(&self, tokens: &[ExprToken]) -> Vec<usize> {
		let mut found = Vec::new();

		for token in tokens {
			found.append(&mut self.find_indexes(token));
		}

		found.sort_unstable();

		if self.is_reversed() {
			found.reverse();
		}

		found
	}

	pub fn find_fn<F>(&self, exp_fn: F) -> Vec<usize> where F: Fn(&ExprToken) -> bool {
		let mut found: Vec<usize> = self.tokens.iter()
			.enumerate()
			.filter(|(i, e)| {
				if !exp_fn(e) {
					false
				} else if self.is_reversed() {
					i <= &self.get_pos()
				} else {
					i >= &self.get_pos()
				}
			})
			.map(|(u, _)| u)
			.collect();

		if self.is_reversed() {
			found.reverse();
		}

		found
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

	pub fn previous(&self) -> Option<&ExprToken> {
		self.get(self.pos - 1)
	}

	pub fn peek(&self) -> Option<&ExprToken> {
		self.get(self.pos)
	}

	pub fn peek_previous(&self) -> Option<&ExprToken> {
		if self.pos == 0 {
			None
		} else {
			self.get(self.pos - 1)
		}
	}

	pub fn remaining(&self) -> &[ExprToken] {
		if self.reversed {
			self.tokens.get(..=self.pos).unwrap()
		} else {
			self.tokens.get(self.pos..).unwrap()
		}
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

impl Iterator for TokenSlicer {
	type Item = ExprToken;

	fn next(&mut self) -> Option<Self::Item> {
		let token = self.tokens.get(self.pos).cloned();

		self.consume(1);

		token
	}
}