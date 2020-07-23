// Constants.
// PI, E, etc..

// TODO: Make constants use Vec<ExprToken> instead of number.
// Will allow for better custom constants.

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

pub type DefaultConsts<'a> = Vec<(&'a str, Decimal)>;

pub const PI: f64 = std::f64::consts::PI;
pub const EULERS_NUMBER: f64 = std::f64::consts::E;


pub fn default_constants<'a>() -> DefaultConsts<'a> {
	vec![
		("PI", dec!(3.14159265358979323846264338327950288)),
		("E", dec!(2.71828182845904523536028747135266250))
	]
}