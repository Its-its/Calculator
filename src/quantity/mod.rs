use std::{ops, fmt};
use std::cmp::{Ordering, PartialOrd};

use crate::{BaseUnit, Result};


pub mod math;
pub mod physics;

pub type FunctionResult = Result<Quantity>;

pub trait FunctionEval: std::fmt::Debug {
	fn eval(&self, params: Vec<Quantity>) -> FunctionResult;
}


#[derive(Debug)]
pub struct Quantity(f64, Option<Box<dyn BaseUnit>>);

impl Quantity {
	pub fn new(value: f64) -> Quantity {
		Quantity(value, None)
	}

	pub fn new_from_base_unit(value: f64, unit: Option<Box<dyn BaseUnit>>) -> Quantity {
		if let Some(unit) = unit {
			Quantity(value / unit.base_factor(), Some(unit))
		} else {
			Quantity(value, None)
		}
	}

	pub fn new_unit(value: f64, unit: Option<Box<dyn BaseUnit>>) -> Quantity {
		Quantity(value, unit)
	}

	pub fn empty() -> Quantity {
		Quantity(0.0, None)
	}

	pub fn pow(self, exp: Quantity) -> Quantity {
		Quantity::new_unit(self.amount().powf(exp.amount()), self.1)
	}


	pub fn amount(&self) -> f64 {
		self.0
	}

	pub fn set_amount(&mut self, value: f64) {
		self.0 = value;
	}

	pub fn total_amount(&self) -> f64 {
		if let Some(unit) = self.unit() {
			self.amount() * unit.base_factor()
		} else {
			self.amount()
		}
	}

	pub fn unit(&self) -> Option<&Box<dyn BaseUnit>> {
		self.1.as_ref()
	}

	pub fn into_unit(self) -> Option<Box<dyn BaseUnit>> {
		self.1
	}


	pub fn this_or_that_fn<F: Fn(f64, f64) -> bool>(self, other: Self, func: F) -> Self {
		if func(self.total_amount(), other.total_amount()) {
			self
		} else {
			other
		}
	}
}


impl fmt::Display for Quantity {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(&format!("{}", self.amount()))?;

		match self.unit() {
			Some(u) => u.fmt(f)?,
			None => ()
		}

		Ok(())
	}
}


impl ops::Add for Quantity {
	type Output = Quantity;

	fn add(self, other: Quantity) -> Self::Output {
		// TODO: New Quantity should use nearest unit.
		// - 900GB + 200GB = 1.1TB
		// - 1h - 30m = 30m

		let total_amount = self.total_amount() + other.total_amount();

		// Return Largest Unit.
		let unit = return_unit(
			self.into_unit(),
			other.into_unit(),
			|v1, v2| std::cmp::max(v1, v2)
		);

		Quantity::new_from_base_unit(total_amount, unit)
	}
}

impl ops::Sub for Quantity {
	type Output = Quantity;

	fn sub(self, other: Quantity) -> Self::Output {
		Quantity::new_unit(self.amount() - other.amount(), self.1)
	}
}

impl ops::Mul for Quantity {
	type Output = Quantity;

	fn mul(self, other: Quantity) -> Self::Output {
		let total_amount = self.total_amount() * other.total_amount();

		// Return Largest Unit.
		let unit = return_unit(
			self.into_unit(),
			other.into_unit(),
			|v1, v2| std::cmp::max(v1, v2)
		);

		Quantity::new_from_base_unit(total_amount, unit)
	}
}

impl ops::Div for Quantity {
	type Output = Quantity;

	fn div(self, other: Quantity) -> Self::Output {
		Quantity::new_unit(self.amount() / other.amount(), self.1)
	}
}

impl PartialOrd for Quantity {
	fn partial_cmp(&self, other: &Quantity) -> Option<Ordering> {
		Some(
			if self.amount() == other.amount() {
				Ordering::Equal
			} else if self.amount() > other.amount() {
				Ordering::Greater
			} else {
				Ordering::Less
			}
		)
	}
}

impl PartialEq for Quantity {
	fn eq(&self, other: &Quantity) -> bool {
		self.amount() == other.amount()
	}
}


fn return_unit<E, F>(u1: Option<E>, u2: Option<E>, func: F) -> Option<E> where F: FnOnce(E, E) -> E {
	match (u1, u2) {
		(Some(u), None) |
		(None, Some(u)) => Some(u),

		(Some(u1), Some(u2)) => {
			Some(func(u1, u2))
		},

		(None, None) => None
	}
}