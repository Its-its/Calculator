// Constants.
// PI, E, etc..

// TODO: Make constants use Vec<ExprToken> instead of number.
// Will allow for better custom constants.

use rust_decimal::Decimal;

pub type DefaultConsts<'a> = Vec<(&'a str, Decimal)>;

pub const PI: f64 = std::f64::consts::PI;
pub const EULERS_NUMBER: f64 = std::f64::consts::E;


pub fn default_constants<'a>() -> DefaultConsts<'a> {
	vec![
		("PI", Decimal::from_scientific(&format!("{}", PI)).unwrap()),
		("E", Decimal::from_scientific(&format!("{}", EULERS_NUMBER)).unwrap())
	]
}