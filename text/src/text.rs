use rust_decimal_macros::dec;


use conversion_parser::{Value, ExprToken, Operator};
use conversion_parser::tokenizer::RangedType;

use conversion::{BaseUnit, Quantity};

use crate::Matcher;


#[derive(Debug)]
pub enum TextValue {
	Parsed(Value),
	Text(ExprToken)
}


pub struct TextStructure<'a> {
	pub text: &'a str,
	pub parsed: Vec<TextValue>,
	pub compiled: Vec<(RangedType, ExprToken)>
}

impl<'a> TextStructure<'a> {
	pub fn new(text: &'a str, parsed: Vec<TextValue>, compiled: Vec<(RangedType, ExprToken)>) -> Self {
		Self { text, parsed, compiled }
	}

	pub fn find<B: BaseUnit>(&self, unit: &B) -> Vec<&TextValue> {
		self.parsed.iter()
		.filter(|v| if let TextValue::Parsed(v) = v {
			v.as_base_unit().map(|u| u.base().base_unit()) == Some(unit.base_unit())
		} else {
			false
		})
		.collect()
	}

	pub fn find_single_with_op<B: BaseUnit, Q: Into<Quantity>>(&self, unit: &B, quantity: Q, op: Operator) -> Option<&TextValue> {
		// Make it into a Value for comparisons.
		let value = Value::Quantity(quantity.into());

		self.find(unit)
		.into_iter()
		.find(|v| if let TextValue::Parsed(v) = v {
			if v.amount().is_some() {
				Value::try_comparison(v.clone(), value.clone(), &op)
				.map(|v| v.amount().unwrap_or_default() == dec!(1.0))
				.unwrap_or_default()
			} else {
				false
			}
		} else {
			false
		})
	}

	pub fn find_with_op<B: BaseUnit, Q: Into<Quantity>>(&self, unit: &B, quantity: Q, op: Operator) -> Vec<&TextValue> {
		// Make it into a Value for comparisons.
		let value = Value::Quantity(quantity.into());

		self.find(unit)
		.into_iter()
		.filter(|v| if let TextValue::Parsed(v) = v {
			if v.amount().is_some() {
				Value::try_comparison(v.clone(), value.clone(), &op)
				.map(|v| v.amount().unwrap_or_default() == dec!(1.0))
				.unwrap_or_default()
			} else {
				false
			}
		} else {
			false
		})
		.collect()
	}

	// TODO: GigaByte.eq(Quantity(10, GigaByte)) instead.

	pub fn match_for<B: BaseUnit>(&self, unit: B) -> Matcher<'_, B> {
		Matcher::new(unit, self)
	}
}