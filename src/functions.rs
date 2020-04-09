use conversion::Quantity;

use crate::{Result, Error};

// floor()
// Etc..

pub type FunctionResult = Result<Quantity>;
pub type FunctionParams = Vec<Quantity>;


pub trait FunctionEval: std::fmt::Debug {
	fn eval(params: FunctionParams) -> FunctionResult;
}




macro_rules! single_param_fn {
	($name:ident, $varname:ident) => {
		#[derive(Debug, Clone)]
		pub struct $name;

		impl FunctionEval for $name {
			fn eval(params: FunctionParams) -> FunctionResult {
				let quantity = params.into_iter().next().ok_or(Error::Text("No Quantity in params".to_string()))?;

				Ok(Quantity::new_unit(quantity.amount().$varname(), quantity.into_unit()))
			}
		}
	};
}


single_param_fn!(Floor, floor);
single_param_fn!(Ceil, ceil);
single_param_fn!(Round, round);
single_param_fn!(Trunc, trunc);
single_param_fn!(Fract, fract);
single_param_fn!(Abs, abs);
single_param_fn!(Signum, signum);
single_param_fn!(Sqrt, sqrt);
single_param_fn!(Recip, recip);
single_param_fn!(Deg, to_degrees);
single_param_fn!(Rad, to_radians);

single_param_fn!(Sin, sin);
single_param_fn!(Cos, cos);
single_param_fn!(Tan, tan);
single_param_fn!(Asin, asin);
single_param_fn!(acos, acos);
single_param_fn!(Atan, atan);

single_param_fn!(Sinh, sinh);
single_param_fn!(Cosh, cosh);
single_param_fn!(Tanh, tanh);
single_param_fn!(Asinh, asinh);
single_param_fn!(acosh, acosh);
single_param_fn!(Atanh, atanh);