use conversion::{Quantity, FunctionEval};
use conversion::quantity::math::*;

use crate::{Result, Error};


pub fn default_functions<'a>() -> Vec<(&'a str, Box<dyn FunctionEval>)> {
	vec![
		("min", Box::new(Min)),
		("max", Box::new(Max)),
		("ceil", Box::new(Ceil)),
		("floor", Box::new(Floor)),
		("round", Box::new(Round)),
		("trunc", Box::new(Trunc)),
		("fract", Box::new(Fract)),
		("abs", Box::new(Abs)),
		("signum", Box::new(Signum)),
		("sqrt", Box::new(Sqrt)),
		("exp", Box::new(Exp)),
		("exp2", Box::new(Exp2)),
		("ln", Box::new(NaturalLogarithm)),
		("log2", Box::new(Log2)),
		("log10", Box::new(Log10)),
		("cbrt", Box::new(CubicRoot)),
		("sin", Box::new(Sine)),
		("cos", Box::new(Cosine)),
		("tan", Box::new(Tangent)),
		("asin", Box::new(Arcsine)),
		("atan", Box::new(Arctangent)),
		("acos", Box::new(Arccosine)),
		("sinh", Box::new(HyperbolicSine)),
		("tanh", Box::new(HyperbolicTangent)),
		("cosh", Box::new(HyperbolicCosine)),
		("asinh", Box::new(InverseHyperbolicSine)),
		("atanh", Box::new(InverseHyperbolicTangent)),
		("acosh", Box::new(InverseHyperbolicCosine)),

		("copysign", Box::new(Copysign)),
		("divEuclid", Box::new(DivEuclid)),
		("remEuclid", Box::new(RemEuclid)),
		("powf", Box::new(Pow)),
		("log", Box::new(Log)),
		("hypot", Box::new(Hypot)),
		("atan2", Box::new(Atan2))
	]
}

pub fn get_func_from_literal<'a>(name: &str) -> Option<Box<dyn FunctionEval>> {
	default_functions()
	.into_iter()
	.find(|u| u.0 == name)
	.map(|i| i.1)
}