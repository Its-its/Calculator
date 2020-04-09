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
	eval: &'a str,
	pos: usize,
	tokens: Vec<ExprToken>
}

impl<'a> Parser<'a> {
	pub fn new(eval: &'a str) -> Self {
		Parser {
			tokenizer: Tokenizer::new(eval),
			tokens: Vec::new(),
			pos: 0,
			eval
		}
	}

	pub fn parse(&mut self) -> Result<()> {
		self.tokenize()?;
		self.pemdaize();

		// Parenthesis Exponent Mul Div Add Sub
		let to_parse = self.parse_orders_expression()?;

		println!("Found: {:?}", to_parse);

		if let Some(to_parse) = to_parse {
			println!("Eval: {:?}", to_parse.eval()?);
		}

		Ok(())
	}

	fn tokenize(&mut self) -> Result<()> {
		self.tokens = self.tokenizer.parse()?;

		println!("Tokenize: {:?}", self.tokens);

		Ok(())
	}


	fn pemdaize(&mut self) {
		let cached = self.tokens.to_vec();
		let found: Vec<(usize, ExprToken)> = cached.iter()
			.enumerate()
			.filter(|(_, e)| e.is_expr_operator())
			.map(|(u, v)| (u, v.clone()))
			.collect();

		println!("Pemdaize: {:?}", found);

		println!("");
	}


	// numbers involving powers or square roots
	fn parse_orders_expression<'b>(&mut self) -> ExpressionResult<'b> {
		let left_expr = self.parse_equality_expression()?;

		Ok(left_expr)
	}

	// == !=
	fn parse_equality_expression<'b>(&mut self) -> ExpressionResult<'b> {
		let left_expr = self.parse_relational_expression()?;

		if self.consume_if_next(&Operator::Equal.into()) {
			//
		}

		if self.consume_if_next(&Operator::DoesNotEqual.into()) {
			//
		}

		Ok(left_expr)
	}

	// < > <= >=
	fn parse_relational_expression<'b>(&mut self) -> ExpressionResult<'b> {
		let left_expr = self.parse_additive_expression()?;

		Ok(left_expr)
	}

	// + -
	fn parse_additive_expression<'b>(&mut self) -> ExpressionResult<'b> {
		let left_expr = self.parse_multiplicative_expression()?;

		if self.consume_if_next(&Operator::Plus.into()) {
			let right_expr = self.parse_multiplicative_expression()?;

			return Ok(Some(Box::new(
				Add::new(
					left_expr.ok_or(Error::InputEmpty)?,
					right_expr.ok_or(Error::InputEmpty)?
				)
			)));
		}

		if self.consume_if_next(&Operator::Minus.into()) {
			let right_expr = self.parse_multiplicative_expression()?;

			return Ok(Some(Box::new(
				Subtract::new(
					left_expr.ok_or(Error::InputEmpty)?,
					right_expr.ok_or(Error::InputEmpty)?
				)
			)));
		}

		Ok(left_expr)
	}

	// % * /
	fn parse_multiplicative_expression<'b>(&mut self) -> ExpressionResult<'b> {
		let left_expr = self.parse_parentheses_expression()?;

		if self.consume_if_next(&Operator::Divide.into()) {
			let right_expr = self.parse_additive_expression()?;

			return Ok(Some(Box::new(
				Divide::new(
					left_expr.ok_or(Error::InputEmpty)?,
					right_expr.ok_or(Error::InputEmpty)?
				)
			)));
		}

		if self.consume_if_next(&Operator::Multiply.into()) {
			let right_expr = self.parse_additive_expression()?;

			return Ok(Some(Box::new(
				Multiply::new(
					left_expr.ok_or(Error::InputEmpty)?,
					right_expr.ok_or(Error::InputEmpty)?
				)
			)));
		}

		Ok(left_expr)
	}

	// ( { [ ] } )
	fn parse_parentheses_expression<'b>(&mut self) -> ExpressionResult<'b> {
		if self.consume_if_next(&ExprToken::StartGrouping) {
			let inner = self.parse_equality_expression()?;

			self.consume_token(&ExprToken::EndGrouping)?;

			Ok(Some(Box::new(
				Grouping::new(inner.ok_or(Error::InputEmpty)?)
			)))
		} else {
			self.parse_number_expression()
		}
	}

	fn parse_number_expression<'b>(&mut self) -> ExpressionResult<'b> {
		if self.is_next_value_func(|v| v.is_number()) {
			let value = return_value!(self, ExprToken::Number);
			let unit = self.parse_unit_expression()?;

			Ok(Some(Box::new(Literal::new(Value::Quantity(Quantity::new_unit(value, unit))))))
		} else {
			Ok(None)
		}
	}

	fn parse_unit_expression<'b>(&mut self) -> Result<Option<BaseUnit<'b>>> {
		if self.is_next_value_func(|v| v.is_literal()) {
			let val = return_value!(self, ExprToken::Literal);

			//
		}

		Ok(None)
	}


	//

	pub fn next(&mut self) -> Option<ExprToken> {
		let token = self.tokens.get(self.pos).cloned();

		self.consume(1);

		token
	}

	pub fn peek(&self) -> Option<&ExprToken> {
		self.tokens.get(self.pos)
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

	pub fn consume(&mut self, amount: usize) {
		self.pos += amount;
	}
}