// https://en.wikipedia.org/wiki/Metric_prefix
// https://en.wikipedia.org/wiki/Non-SI_units_mentioned_in_the_SI


pub mod si;
pub mod time;
pub mod data;
pub mod imperial;

pub use si::*;
pub use time::*;
pub use data::*;
pub use imperial::*;


#[macro_export]
macro_rules! match_conv {
	($typeName:expr, [$($unitName:ty = $amount:expr),+]) => {
		match $typeName {
			$($unitName => Some($amount)),*
			_ => None
		}
	};
}


#[macro_export]
macro_rules! create_non_standard_unit {
	($unitName:ident, $baseUnit:expr, $factor:expr, $longName:expr, $shortName:expr) => {
		#[derive(Debug, Clone)]
		pub struct $unitName;

		impl BaseUnit for $unitName {
			fn long(&self) -> &str {
				$longName
			}

			fn short(&self) -> Option<&str> {
				Some($shortName)
			}

			fn alt(&self) -> Option<&str> {
				None
			}

			fn conversion_factor_for(&self, _unit: &dyn BaseUnit) -> Option<f64> {
				Some($factor)
			}

			fn base_unit(&self) -> Option<&dyn BaseUnit> {
				Some(&$baseUnit)
			}
		}
	};
}

#[macro_export]
macro_rules! create_standard_unit {
	($unitName:ident, $longName:expr, $shortName:expr) => {
		create_standard_unit!($unitName, $longName, $shortName, []);
	};

	($unitName:ident, $longName:expr, $shortName:expr, [ $($otherUnitName:ty = $amount:expr),* ]) => {
		#[derive(Debug, Clone)]
		pub struct $unitName;

		impl BaseUnit for $unitName {
			fn long(&self) -> &str {
				$longName
			}

			fn short(&self) -> Option<&str> {
				Some($shortName)
			}

			fn alt(&self) -> Option<&str> {
				None
			}

			fn conversion_factor_for(&self, _unit: &dyn BaseUnit) -> Option<f64> {
				Some(1.0)
			}

			fn base_unit(&self) -> Option<&dyn BaseUnit> {
				None
			}
		}
	};
}


pub trait BaseUnit: std::fmt::Debug {
	fn long(&self) -> &str;
	fn short(&self) -> Option<&str>;
	fn alt(&self) -> Option<&str>;

	fn base_factor(&self) -> f64 {
		1.0
	}

	fn base_unit(&self) -> Option<&dyn BaseUnit> {
		None
	}

	fn conversion_factor_for(&self, _unit: &dyn BaseUnit) -> Option<f64> {
		None
	}

	fn can_convert_to(&self, unit: &dyn BaseUnit) -> bool {
		self.conversion_factor_for(unit).is_some()
	}
}

impl PartialEq for dyn BaseUnit {
	fn eq(&self, other: &dyn BaseUnit) -> bool {
		self.long() == other.long()
	}
}


pub fn is_convertable(from: &dyn BaseUnit, to: &dyn BaseUnit) -> bool {
	from.base_unit()
	.unwrap_or(from)
	.can_convert_to(to)
}