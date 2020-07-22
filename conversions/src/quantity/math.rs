// Basic Math Functions

use crate::{Error, Result};
use super::{Quantity, FunctionEval, FunctionResult};

// TODO: Ensure params are all same base type.


macro_rules! create_single {
	($struc:ident, $func:ident) => {
		#[derive(Debug, Clone)]
		pub struct $struc;

		impl FunctionEval for $struc {
			fn eval(&self, params: Vec<Quantity>) -> FunctionResult {
				let mut iter = params.into_iter();

				let mut last_item = next(&mut iter)?;

				last_item.set_amount(last_item.amount().$func());

				Ok(last_item)
			}
		}
	};
}

macro_rules! create_double {
	($struc:ident, $func:ident) => {
		#[derive(Debug, Clone)]
		pub struct $struc;

		impl FunctionEval for $struc {
			fn eval(&self, params: Vec<Quantity>) -> FunctionResult {
				let mut iter = params.into_iter();

				let mut first = next(&mut iter)?;
				let second = next(&mut iter)?;

				first.set_amount(first.amount().$func(second.amount()));

				Ok(first)
			}
		}
	};
}


#[derive(Debug, Clone)]
pub struct Min;

impl FunctionEval for Min {
	fn eval(&self, params: Vec<Quantity>) -> FunctionResult {
		let mut iter = params.into_iter();

		let mut last_item = next(&mut iter)?;

		while let Some(other) = iter.next() {
			last_item = last_item.this_or_that_fn(other, |a, b| a.min(b) == a)
		}

		Ok(last_item)
	}
}


#[derive(Debug, Clone)]
pub struct Max;

impl FunctionEval for Max {
	fn eval(&self, params: Vec<Quantity>) -> FunctionResult {
		let mut iter = params.into_iter();

		let mut last_item = next(&mut iter)?;

		while let Some(other) = iter.next() {
			last_item = last_item.this_or_that_fn(other, |a, b| a.max(b) == a)
		}

		Ok(last_item)
	}
}



create_single!(Ceil, ceil);
create_single!(Floor, floor);
create_single!(Round, round);
create_single!(Trunc, trunc);
create_single!(Fract, fract);
create_single!(Abs, abs);
create_single!(Signum, signum);
create_single!(Sqrt, sqrt);
create_single!(Exp, exp);
create_single!(Exp2, exp2);
create_single!(NaturalLogarithm, ln);
create_single!(Log2, log2);
create_single!(Log10, log10);
create_single!(CubicRoot, cbrt);
create_single!(Sine, sin);
create_single!(Cosine, cos);
create_single!(Tangent, tan);
create_single!(Arcsine, asin);
create_single!(Arctangent, atan);
create_single!(Arccosine, acos);
create_single!(HyperbolicSine, sinh);
create_single!(HyperbolicTangent, tanh);
create_single!(HyperbolicCosine, cosh);
create_single!(InverseHyperbolicSine, asinh);
create_single!(InverseHyperbolicTangent, atanh);
create_single!(InverseHyperbolicCosine, acosh);

create_double!(Copysign, copysign);
create_double!(DivEuclid, div_euclid);
create_double!(RemEuclid, rem_euclid);
create_double!(Pow, powf);
create_double!(Log, log);
create_double!(Hypot, hypot);
create_double!(Atan2, atan2);





fn next<F: Iterator<Item = Quantity>>(params: &mut F) -> Result<Quantity> {
	params.next()
	.ok_or(Error::ExpectedArgument)
}