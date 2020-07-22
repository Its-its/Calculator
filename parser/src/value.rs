use std::{fmt, ops};

use conversion::{Quantity, BaseUnit, Units};

use crate::{Result, Error, ExprToken, Operator};
use crate::units::convert;


#[derive(Debug)]
pub enum Value {
	Quantity(Quantity),
	Unit(Units)
}

impl Value {
	pub fn new_quantity(value: f64) -> Value {
		Value::Quantity(Quantity::new(value))
	}

	pub fn new_quantity_unit(value: f64, unit: Option<Units>) -> Value {
		Value::Quantity(Quantity::new_unit(value, unit))
	}


	pub fn as_base_unit(&self) -> Option<&Units> {
		match self {
			Value::Quantity(q) => q.unit(),
			Value::Unit(u) => Some(u)
		}
	}

	pub fn clone_base_unit(&self) -> Option<Units> {
		match self {
			Value::Quantity(q) => q.unit().cloned(),
			Value::Unit(u) => Some(u.clone())
		}
	}

	pub fn base_factor(&self) -> f64 {
		match self {
			Value::Quantity(q) => q.unit().map(|u| u.base().base_factor()).unwrap_or(1.0),
			Value::Unit(u) => u.base().base_factor()
		}
	}

	pub fn amount(&self) -> Option<f64> {
		match self {
			Value::Quantity(q) => Some(q.amount()),
			Value::Unit(_) => None
		}
	}

	pub fn total_amount(&self) -> Option<f64> {
		match self {
			Value::Quantity(q) => Some(q.total_amount()),
			Value::Unit(_) => None
		}
	}

	pub fn into_tokens(self) -> Vec<ExprToken> {
		let mut tokens = Vec::new();

		match self {
			Value::Quantity(q) => {
				tokens.push(ExprToken::Number(q.amount()));

				if let Some(unit) = q.into_unit() {
					tokens.push(ExprToken::Literal(unit.short()));
				}
			}

			Value::Unit(unit) => {
				tokens.push(ExprToken::Literal(unit.short()));
			}
		}

		tokens
	}

	pub fn into_quantity(self) -> Option<Quantity> {
		match self {
			Value::Quantity(q) => Some(q),
			_ => None
		}
	}

	pub fn into_base_unit(self) -> Option<Units> {
		match self {
			Value::Quantity(q) => q.into_unit(),
			Value::Unit(u) => Some(u)
		}
	}


	pub fn try_add(left: Value, right: Value) -> Result<Value> {
		match (left, right) {
			(Value::Quantity(left), Value::Quantity(right)) => {
				let (l_amount, r_amount) = (left.amount(), right.amount());
				let (l_name, r_name) = (
					left.unit().map(|u| u.short()).unwrap_or_default(),
					right.unit().map(|u| u.short()).unwrap_or_default()
				);

				let value = left + right;

				print_dbg!(
					"Add: {}{} + {}{} = {}{}",
					l_amount,
					l_name,
					r_amount,
					r_name,
					value.amount(),
					value.unit().map(|u| u.short()).unwrap_or_default()
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
				print_dbg!("Sub: {} - {} = {}", l_amount, r_amount, value.amount());
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
				print_dbg!("Mul: {} * {} = {}", l_amount, r_amount, value.amount());
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

				print_dbg!("Div: {} / {} = {}", l_amount, r_amount, value.amount());

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

				print_dbg!("Exp: {}^{} = {}", l_amount, r_amount, value.amount());

				Ok(Value::Quantity(value))
			}

			_ => Err("Unable to divide.".into())
		}
	}

	pub fn try_conversion(left: Value, right: Value) -> Result<Value> {
		let (l_amount, r_amount) = (left.amount(), right.amount());

		let unit = right.clone_base_unit();

		let value = convert(&left, &right)?;

		print_dbg!("Conv: {}(f {}) -> {}(f {}) = {}", l_amount.unwrap_or_default(), left.base_factor(), r_amount.unwrap_or_default(), right.base_factor(), value);

		Ok(Value::Quantity(Quantity::new_unit(value, unit)))
	}

	pub fn try_comparison(left: Value, right: Value, op: &Operator) -> Result<Value> {
		let (l_amount, r_amount) = (left.total_amount(), right.total_amount());

		let value = match op {
			Operator::GreaterThan => (l_amount > r_amount) as usize as f64,
			Operator::GreaterThanOrEqual => (l_amount >= r_amount) as usize as f64,
			Operator::LessThan => (l_amount < r_amount) as usize as f64,
			Operator::LessThanOrEqual => (l_amount <= r_amount) as usize as f64,
			Operator::DoubleEqual => (l_amount == r_amount) as usize as f64,
			Operator::DoesNotEqual => (l_amount != r_amount) as usize as f64,
			_ => return Err(Error::Text("Invalid Operator when trying to compare".into()))
		};

		print_dbg!("Comp: {} {} {} = {}", l_amount.clone().unwrap_or_default(), op, r_amount.clone().unwrap_or_default(), value);

		Ok(Value::Quantity(Quantity::new(value)))
	}
}

impl PartialEq for Value {
	fn eq(&self, other: &Value) -> bool {
		match (self, other) {
			(Value::Quantity(q1), Value::Quantity(q2)) => q1 == q2,
			_ => false
		}
	}
}

impl Clone for Value {
	fn clone(&self) -> Self {
		match self {
			Value::Quantity(q) => Value::Quantity(Quantity::new_unit(q.amount(), q.unit().cloned())),
			Value::Unit(u) => Value::Unit(u.clone())
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