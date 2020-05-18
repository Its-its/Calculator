use std::{ops, fmt, cmp};
use std::cmp::{Ordering, PartialOrd};

use crate::{BaseUnit, Result};


pub mod math;
pub mod physics;

pub type FunctionResult = Result<Quantity>;


// Adds the ability to clone Box<dyn FunctionEval>
pub trait CloneFunctionEval {
	fn clone_fn_eval(&self) -> Box<dyn FunctionEval>;
}

impl<T> CloneFunctionEval for T where T: FunctionEval + Clone + 'static {
	fn clone_fn_eval(&self) -> Box<dyn FunctionEval> {
		Box::new(self.clone())
	}
}

impl Clone for Box<dyn FunctionEval> {
	fn clone(&self) -> Self {
		self.clone_fn_eval()
	}
}

pub trait FunctionEval: fmt::Debug + CloneFunctionEval {
	fn eval(&self, params: Vec<Quantity>) -> FunctionResult;
}


#[derive(Debug, Clone)]
pub struct Quantity(f64, Option<Units>);

impl Quantity {
	pub fn new(value: f64) -> Quantity {
		Quantity(value, None)
	}

	pub fn new_from_base_unit(value: f64, unit: Option<Units>) -> Quantity {
		if let Some(unit) = unit {
			Quantity(value / unit.base().base_factor(), Some(unit))
		} else {
			Quantity(value, None)
		}
	}

	pub fn new_unit(value: f64, unit: Option<Units>) -> Quantity {
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
			self.amount() * unit.base().base_factor()
		} else {
			self.amount()
		}
	}

	pub fn unit(&self) -> Option<&Units> {
		self.1.as_ref()
	}

	pub fn into_unit(self) -> Option<Units> {
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
			Some(u) => {
				f.write_str(" ")?;

				u.fmt(f)?
			},
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




// Units.
// Ex: 1 GB or 1 GB/s

#[derive(Debug, Clone)]
pub struct Units(Vec<Box<dyn BaseUnit>>);

impl Units {
	pub fn new(unit: Box<dyn BaseUnit>) -> Self {
		Self(vec![unit])
	}

	pub fn new_2(unit: Box<dyn BaseUnit>, unit2: Box<dyn BaseUnit>) -> Self {
		Self(vec![unit, unit2])
	}

	pub fn new_vec(units: Vec<Box<dyn BaseUnit>>) -> Self {
		Self(units)
	}


	pub fn base(&self) -> &Box<dyn BaseUnit> {
		self.0.first().unwrap()
	}

	pub fn base_2(&self) -> Option<&Box<dyn BaseUnit>> {
		self.0.get(1)
	}

	pub fn is_base_equal(&self, other: &Units) -> bool {
		self.base() == other.base()
	}

	pub fn is_base_2_equal(&self, other: &Units) -> bool {
		self.base_2() == other.base_2()
	}

	pub fn total_factor(&self) -> f64 {
		let base = self.base().base_factor();

		if let Some(div) = self.base_2() {
			base / div.base_factor()
		} else {
			base
		}
	}

	pub fn long(&self) -> String {
		let base = self.base().long();

		if let Some(div) = self.base_2() {
			format!("{}/{}", base, div.long())
		} else {
			base.to_string()
		}
	}

	pub fn short(&self) -> String {
		let short = self.base().short().unwrap_or(self.base().long());

		if let Some(div) = self.base_2() {
			format!("{}/{}", short, div.short().unwrap_or(div.long()))
		} else {
			short.to_string()
		}
	}
}

impl fmt::Display for Units {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for (i, u) in self.0.iter().enumerate() {
			if i != 0 {
				f.write_str("/")?;
			}

			u.fmt(f)?
		}

		Ok(())
	}
}



impl PartialEq for Units {
	fn eq(&self, other: &Units) -> bool {
		if self.base() == other.base() {
			match (self.base_2(), other.base_2()) {
				(Some(u1), Some(u2)) => u1 == u2,
				(None, None) => true,
				_ => false
			}
		} else {
			false
		}
	}
}

impl PartialOrd for Units {
	fn partial_cmp(&self, other: &Units) -> Option<cmp::Ordering> {
		Some(
			if self.total_factor() > other.total_factor() {
				cmp::Ordering::Greater
			} else if self.total_factor() > other.total_factor() {
				cmp::Ordering::Less
			} else {
				cmp::Ordering::Equal
			}
		)
	}
}

impl Ord for Units {
	fn cmp(&self, other: &Units) -> cmp::Ordering {
		if self.total_factor() > other.total_factor() {
			cmp::Ordering::Greater
		} else if self.total_factor() > other.total_factor() {
			cmp::Ordering::Less
		} else {
			cmp::Ordering::Equal
		}
	}
}

impl Eq for Units {}