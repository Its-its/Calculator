use std::fmt;

use conversion::{Quantity, FunctionEval};

use crate::{Result, Error, Value, Operator};


pub type ExpressionArg = Box<dyn Expression>;

pub trait Expression: fmt::Debug {
	fn eval(&self) -> Result<Value>;
}



#[derive(Debug)]
pub struct Add(ExpressionArg, ExpressionArg);

impl Add {
	pub fn new(left: ExpressionArg, right: ExpressionArg) -> Self {
		Add(left, right)
	}
}

impl Expression for Add {
	fn eval(&self) -> Result<Value> {
		let left = self.0.eval()?;
		let right = self.1.eval()?;

		Ok(Value::try_add(left, right)?)
	}
}



#[derive(Debug)]
pub struct Subtract(ExpressionArg, ExpressionArg);

impl Subtract {
	pub fn new(left: ExpressionArg, right: ExpressionArg) -> Self {
		Subtract(left, right)
	}
}

impl Expression for Subtract {
	fn eval(&self) -> Result<Value> {
		let left = self.0.eval()?;
		let right = self.1.eval()?;

		Ok(Value::try_subtract(left, right)?)
	}
}



#[derive(Debug)]
pub struct Multiply(ExpressionArg, ExpressionArg);

impl Multiply {
	pub fn new(left: ExpressionArg, right: ExpressionArg) -> Self {
		Multiply(left, right)
	}
}

impl Expression for Multiply {
	fn eval(&self) -> Result<Value> {
		let left = self.0.eval()?;
		let right = self.1.eval()?;

		Ok(Value::try_multiply(left, right)?)
	}
}



#[derive(Debug)]
pub struct Divide(ExpressionArg, ExpressionArg);

impl Divide {
	pub fn new(left: ExpressionArg, right: ExpressionArg) -> Self {
		Divide(left, right)
	}
}

impl Expression for Divide {
	fn eval(&self) -> Result<Value> {
		let left = self.0.eval()?;
		let right = self.1.eval()?;

		Ok(Value::try_divide(left, right)?)
	}
}



// #[derive(Debug)]
// pub struct Exponentiate(ExpressionArg, ExpressionArg);

// impl Exponentiate {
// 	pub fn new(left: ExpressionArg, right: ExpressionArg) -> Self {
// 		Exponentiate(left, right)
// 	}
// }

// impl Expression for Exponentiate {
// 	fn eval(&self) -> Result<Value> {
// 		let left = self.0.eval()?;
// 		let right = self.1.eval()?;

// 		Ok(Value::try_exponentiate(left, right)?)
// 	}
// }



#[derive(Debug)]
pub struct Conversion(ExpressionArg, ExpressionArg);

impl Conversion {
	pub fn new(left: ExpressionArg, right: ExpressionArg) -> Self {
		Conversion(left, right)
	}
}

impl Expression for Conversion {
	fn eval(&self) -> Result<Value> {
		let left = self.0.eval()?;
		let right = self.1.eval()?;

		Ok(Value::try_conversion(left, right)?)
	}
}



#[derive(Debug)]
pub struct Comparison(ExpressionArg, ExpressionArg, Operator);

impl Comparison {
	pub fn new(left: ExpressionArg, right: ExpressionArg, op: Operator) -> Self {
		Comparison(left, right, op)
	}
}

impl Expression for Comparison {
	fn eval(&self) -> Result<Value> {
		let left = self.0.eval()?;
		let right = self.1.eval()?;

		Ok(Value::try_comparison(left, right, &self.2)?)
	}
}



#[derive(Debug)]
pub struct Function(Box<dyn FunctionEval>, Vec<ExpressionArg>);

impl Function {
	pub fn new(func: Box<dyn FunctionEval>, args: Vec<ExpressionArg>) -> Self {
		Function(func, args)
	}
}

impl Expression for Function {
	fn eval(&self) -> Result<Value> {
		let params = self.1.iter()
			.map(|i| i.eval())
			.collect::<Result<Vec<Value>>>()?;

		let params = params.into_iter()
			.map(|i| i.into_quantity().ok_or_else(|| Error::ExpectedQuantity))
			.collect::<Result<Vec<Quantity>>>()?;

		Ok(Value::Quantity(self.0.eval(params)?))
	}
}



#[derive(Debug)]
pub struct Literal(Value);

impl Literal {
	pub fn new(value: Value) -> Self {
		Literal(value)
	}
}

impl Expression for Literal {
	fn eval(&self) -> Result<Value> {
		Ok(self.0.clone())
	}
}



#[derive(Debug)]
pub struct Grouping(ExpressionArg);

impl Grouping {
	pub fn new(value: ExpressionArg) -> Self {
		Grouping(value)
	}
}

impl Expression for Grouping {
	fn eval(&self) -> Result<Value> {
		Ok(self.0.eval()?)
	}
}