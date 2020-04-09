use std::{fmt, ops};

use conversion::{Quantity, BaseUnit};

use crate::{Result, Error, ExprToken};


#[derive(Debug)]
pub enum Value {
	Quantity(Quantity),
	Unit(Box<dyn BaseUnit>)
}

impl Value {
	pub fn as_base_unit(&self) -> Option<&Box<dyn BaseUnit>> {
		match self {
			Value::Quantity(q) => q.unit(),
			Value::Unit(u) => Some(u)
		}
	}

	pub fn amount(&self) -> Option<f64> {
		match self {
			Value::Quantity(q) => Some(q.amount()),
			Value::Unit(_) => None
		}
	}

	pub fn into_tokens(self) -> Vec<ExprToken> {
		let mut tokens = Vec::new();

		match self {
			Value::Quantity(q) => {
				tokens.push(ExprToken::Number(q.amount()));

				if let Some(unit) = q.into_unit() {
					tokens.push(ExprToken::Literal(unit.short().to_owned().unwrap_or_else(|| unit.long()).to_string()));
				}
			}

			Value::Unit(unit) => {
				tokens.push(ExprToken::Literal(unit.short().to_owned().unwrap_or_else(|| unit.long()).to_string()));
			}
		}

		tokens
	}


	pub fn try_add(left: Value, right: Value) -> Result<Value> {
		match (left, right) {
			(Value::Quantity(left), Value::Quantity(right)) => {
				let (l_amount, r_amount) = (left.amount(), right.amount());
				let (l_name, r_name) = (
					left.unit().map(|u| u.short().unwrap_or(u.long()).to_string()).unwrap_or_default(),
					right.unit().map(|u| u.short().unwrap_or(u.long()).to_string()).unwrap_or_default()
				);

				let value = left + right;

				println!(
					"Add: {}{} + {}{} = {}",
					l_amount,
					l_name,
					r_amount,
					r_name,
					value.amount()
				);

				Ok(Value::Quantity(value))
			}

			_ => Err("Unable to add.".into())
		}
	}

	pub fn try_subtract(left: Value, right: Value) -> Result<Value> {
		match (left, right) {
			(Value::Quantity(left), Value::Quantity(right)) => {
				let (l_amount, r_amount) = (left.amount(), right.amount());

				let value = left - right;
				println!("Sub: {} - {} = {}", l_amount, r_amount, value.amount());
				Ok(Value::Quantity(value))
			}

			_ => Err("Unable to subtract.".into())
		}
	}

	pub fn try_multiply(left: Value, right: Value) -> Result<Value> {
		match (left, right) {
			(Value::Quantity(left), Value::Quantity(right)) => {
				let (l_amount, r_amount) = (left.amount(), right.amount());

				let value = left * right;
				println!("Mul: {} * {} = {}", l_amount, r_amount, value.amount());
				Ok(Value::Quantity(value))
			}

			_ => Err("Unable to multiply.".into())
		}
	}

	pub fn try_divide(left: Value, right: Value) -> Result<Value> {
		match (left, right) {
			(Value::Quantity(left), Value::Quantity(right)) => {
				let (l_amount, r_amount) = (left.amount(), right.amount());

				let value = left / right;

				println!("Div: {} / {} = {}", l_amount, r_amount, value.amount());

				Ok(Value::Quantity(value))
			}

			_ => Err("Unable to divide.".into())
		}
	}

	pub fn try_exponentiate(left: Value, right: Value) -> Result<Value> {
		match (left, right) {
			(Value::Quantity(left), Value::Quantity(right)) => {
				let (l_amount, r_amount) = (left.amount(), right.amount());

				let value = left.pow(right);

				println!("Exp: {}^{} = {}", l_amount, r_amount, value.amount());

				Ok(Value::Quantity(value))
			}

			_ => Err("Unable to divide.".into())
		}
	}

	// Unimplemented

	pub fn try_equals(left: Value, right: Value) -> Result<Value> {
		match (left, right) {
			(Value::Quantity(left), Value::Quantity(right)) => {
				Err("Unable to equal.".into())
			}

			_ => Err("Unable to equal.".into())
		}
	}
}

impl PartialEq for Value {
	fn eq(&self, other: &Value) -> bool {
		match (self, other) {
			(Value::Quantity(q1), Value::Quantity(q2)) => q1 == q2,
			//
			_ => false
		}
	}
}

impl Clone for Value {
	fn clone(&self) -> Self {
		match self {
			Value::Quantity(q) => Value::Quantity(Quantity::new_unit(q.amount(), q.unit().map(|i| crate::units::find_unit(i)))),
			Value::Unit(u) => Value::Unit(crate::units::find_unit(u))
		}
	}
}

impl fmt::Display for Value {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Value::Quantity(q) => q.fmt(f),
			Value::Unit(u) => u.fmt(f)
		}
	}
}