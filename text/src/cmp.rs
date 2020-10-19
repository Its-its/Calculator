use conversion::{BaseUnit, Quantity};


// pub trait EqualityForAll: BaseUnit {
// 	fn eq(self, other: impl BaseUnit) -> bool {
// 		true
// 	}
// }

// impl<T> EqualityForAll for T where T: BaseUnit + 'static {}

pub struct Gt(Box<dyn BaseUnit>, Quantity);

impl Gt {
	pub fn new(unit: Box<dyn BaseUnit>, quantity: Quantity) -> Self {
		Gt(unit, quantity)
	}
}