use std::fmt;

use conversion::{
	Quantity, BaseUnit
};

use crate::{Result, Value};


pub type ExpressionArg = Box<dyn Expression>;
pub type CallFunction = fn(ExpressionArg, ExpressionArg) -> ExpressionArg;


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



#[derive(Debug)]
pub struct Exponentiate(ExpressionArg, ExpressionArg);

impl Exponentiate {
	pub fn new(left: ExpressionArg, right: ExpressionArg) -> Self {
		Exponentiate(left, right)
	}
}


impl Expression for Exponentiate {
	fn eval(&self) -> Result<Value> {
		let left = self.0.eval()?;
		let right = self.1.eval()?;

		Ok(Value::try_exponentiate(left, right)?)
	}
}



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