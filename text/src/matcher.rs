use rust_decimal_macros::dec;

use conversion_parser::{Value, ExprToken, Operator};
use conversion_parser::tokenizer::RangedType;

use conversion::{BaseUnit, Quantity};

use crate::{TextStructure, TextValue, Label};

pub struct Matcher<'a, B: BaseUnit + 'static> {
	pub unit: B,
	pub structure: &'a TextStructure<'a>,

	pub quantity: Option<Quantity>,
	pub label: Option<Label>
}

impl<'a, B: BaseUnit + 'static> Matcher<'a, B> {
	pub fn new(unit: B, structure: &'a TextStructure<'_>) -> Self {
		Self {
			unit,
			structure,
			quantity: None,
			label: None
		}
	}


	pub fn amount<Q: Into<Quantity>>(&mut self, quantity: Q) -> &mut Self {
		self.quantity = Some(quantity.into());

		self
	}

	pub fn label(&mut self, label: Label) -> &mut Self {
		self.label = Some(label);

		self
	}

	//

	pub fn is_equal(&self) -> MatchFinder<'_, B> {
		MatchFinder::new(self, Operator::DoubleEqual)
	}

	pub fn is_not_equal(&self) -> MatchFinder<'_, B> {
		MatchFinder::new(self, Operator::DoesNotEqual)
	}

	pub fn is_greater_than(&self) -> MatchFinder<'_, B> {
		MatchFinder::new(self, Operator::GreaterThan)
	}

	pub fn is_greater_than_or_equal(&self) -> MatchFinder<'_, B> {
		MatchFinder::new(self, Operator::GreaterThanOrEqual)
	}

	pub fn is_less_than(&self) -> MatchFinder<'_, B> {
		MatchFinder::new(self, Operator::LessThan)
	}

	pub fn is_less_than_or_equal(&self) -> MatchFinder<'_, B> {
		MatchFinder::new(self, Operator::LessThanOrEqual)
	}
}

pub struct MatchFinder<'a, B: BaseUnit + 'static> {
	matcher: &'a Matcher<'a, B>,

	operator: Operator
}

impl<'a, B: BaseUnit + 'static> MatchFinder<'a, B> {
	pub fn new(matcher: &'a Matcher<'a, B>, operator: Operator, ) -> Self {
		Self {
			matcher,
			operator
		}
	}

	pub fn was_found(self) -> bool {
		if let Some(quantity) = self.matcher.quantity.as_ref() {
			self.matcher.structure.find_single_with_op(
				&self.matcher.unit,
				quantity.clone(),
				self.operator
			).is_some()
		} else {
			false
		}
	}

	pub fn into_vector(self) -> Vec<&'a TextValue> {
		if let Some(quantity) = self.matcher.quantity.as_ref() {
			self.matcher.structure.find_with_op(
				&self.matcher.unit,
				quantity.clone(),
				self.operator
			)
		} else {
			Vec::new()
		}
	}
}