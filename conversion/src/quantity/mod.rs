use std::{ops, fmt, cmp};
use std::cmp::{Ordering, PartialOrd};

use rust_decimal::Decimal;

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
pub struct Quantity(Decimal, Option<Units>);

impl Quantity {
	pub fn new(value: Decimal) -> Quantity {
		Quantity(value, None)
	}

	pub fn new_from_base_unit(value: Decimal, unit: Option<Units>) -> Quantity {
		if let Some(unit) = unit {
			Quantity(value / unit.base().base_factor(), Some(unit))
		} else {
			Quantity(value, None)
		}
	}

	pub fn new_unit(value: Decimal, unit: Option<Units>) -> Quantity {
		Quantity(value, unit)
	}

	pub fn empty() -> Quantity {
		Quantity(Decimal::default(), None)
	}

	// pub fn pow(self, exp: Quantity) -> Quantity {
	// 	Quantity::new_unit(self.amount().powf(exp.amount()), self.1)
	// }


	pub fn amount(&self) -> Decimal {
		self.0
	}

	pub fn set_amount(&mut self, value: Decimal) {
		self.0 = value;
	}

	pub fn total_amount(&self) -> Decimal {
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

	pub fn remove_units(&mut self) {
		self.1 = None;
	}


	pub fn this_or_that_fn<F: Fn(Decimal, Decimal) -> bool>(self, other: Self, func: F) -> Self {
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

		if let Some(u) = self.unit() {
			f.write_str(" ")?;

			u.fmt(f)?;
		}

		Ok(())
	}
}


impl ops::Add for Quantity {
	type Output = Quantity;

	fn add(self, mut other: Quantity) -> Self::Output {
		// TODO: New Quantity should use nearest unit.
		// - 900GB + 200GB = 1.1TB
		// - 1h - 30m = 30m

		let total_amount = if other.unit().map(|u| u.base() == "%").unwrap_or_default() {
			// 200 + 20% = 240
			other.remove_units();

			self.total_amount() + (self.total_amount() * (other.total_amount() / Decimal::new(100, 0)))
		} else {
			self.total_amount() + other.total_amount()
		};

		// Return Largest Unit.
		let unit = return_unit(
			self.into_unit(),
			other.into_unit(),
			std::cmp::max
		);

		Quantity::new_from_base_unit(total_amount, unit)
	}
}

impl ops::Sub for Quantity {
	type Output = Quantity;

	fn sub(self, mut other: Quantity) -> Self::Output {
		let total_amount = if other.unit().map(|u| u.base() == "%").unwrap_or_default() {
			// 200 - 20% = 160
			other.remove_units();

			self.total_amount() - (self.total_amount() * (other.total_amount() / Decimal::new(100, 0)))
		} else {
			self.total_amount() - other.total_amount()
		};

		// Return Largest Unit.
		let unit = return_unit(
			self.into_unit(),
			other.into_unit(),
			std::cmp::max
		);

		Quantity::new_from_base_unit(total_amount, unit)
	}
}

impl ops::Mul for Quantity {
	type Output = Quantity;

	fn mul(self, mut other: Quantity) -> Self::Output {
		// TODO: Re-question on if I should actually use total_amount.
		// Could cause issues with types like PB, EXA, etc..

		let total_amount = if other.unit().map(|u| u.base() == "%").unwrap_or_default() {
			// 200 * 20% = 8,000
			other.remove_units();

			self.total_amount() * (self.total_amount() * (other.total_amount() / Decimal::new(100, 0)))
		} else {
			self.total_amount() * other.total_amount()
		};

		// Return Largest Unit.
		let unit = return_unit(
			self.into_unit(),
			other.into_unit(),
			std::cmp::max
		);

		let factor = unit.as_ref().map(|i| i.base().base_factor()).unwrap_or_else(|| Decimal::new(1, 0));

		Quantity::new_from_base_unit(total_amount / factor, unit)
	}
}

impl ops::Div for Quantity {
	type Output = Quantity;

	fn div(self, mut other: Quantity) -> Self::Output {
		let total_amount = if other.unit().map(|u| u.base() == "%").unwrap_or_default() {
			// 200 / 20% = 5
			other.remove_units();

			self.total_amount() / (self.total_amount() * (other.total_amount() / Decimal::new(100, 0)))
		} else {
			self.total_amount() / other.total_amount()
		};

		// Return Largest Unit.
		let unit = return_unit(
			self.into_unit(),
			other.into_unit(),
			std::cmp::max
		);

		let factor = unit.as_ref().map(|i| i.base().base_factor()).unwrap_or_else(|| Decimal::new(1, 0));

		Quantity::new_from_base_unit(total_amount * factor, unit)
	}
}

impl PartialOrd for Quantity {
	fn partial_cmp(&self, other: &Quantity) -> Option<Ordering> {
		self.amount().partial_cmp(&other.amount())
	}
}

impl PartialEq for Quantity {
	fn eq(&self, other: &Quantity) -> bool {
		self.amount().eq(&other.amount())
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

	pub fn total_factor(&self) -> Decimal {
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
		let short = self.base().short().unwrap_or_else(|| self.base().long());

		if let Some(div) = self.base_2() {
			format!("{}/{}", short, div.short().unwrap_or_else(|| div.long()))
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
		self.total_factor().partial_cmp(&other.total_factor())
	}
}

impl Ord for Units {
	fn cmp(&self, other: &Units) -> cmp::Ordering {
		self.total_factor().cmp(&other.total_factor())
	}
}

impl Eq for Units {}