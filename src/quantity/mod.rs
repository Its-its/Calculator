use std::{ops, fmt};
use std::cmp::{Ordering, PartialOrd};

use crate::{BaseUnit, Result};


pub mod math;
pub mod physics;

pub type FunctionResult = Result<Quantity>;
pub type FunctionParams<'a> = &'a [Quantity];


pub trait FunctionEval: std::fmt::Debug {
	fn eval(params: FunctionParams) -> FunctionResult;
}


#[derive(Debug)]
pub struct Quantity(f64, Option<Box<dyn BaseUnit>>);

impl Quantity {
	pub fn new(value: f64) -> Quantity {
		Quantity(value, None)
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
		Quantity::new_unit(self.amount() + other.amount(), self.1)
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
		Quantity::new_unit(self.amount() * other.amount(), self.1)
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