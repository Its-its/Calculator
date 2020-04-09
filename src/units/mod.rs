// https://en.wikipedia.org/wiki/Metric_prefix
// https://en.wikipedia.org/wiki/Non-SI_units_mentioned_in_the_SI

use std::fmt;

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
	($unitName:ident, $baseUnit:expr, $factor:expr, $longName:expr) => {
		create_non_standard_unit!(full $unitName, $baseUnit, $factor, $longName, None);
	};

	($unitName:ident, $baseUnit:expr, $factor:expr, $longName:expr, $shortName:expr) => {
		create_non_standard_unit!(full $unitName, $baseUnit, $factor, $longName, Some($shortName));
	};


	(full $unitName:ident, $baseUnit:expr, $factor:expr, $longName:expr, $shortName:expr) => {
		#[derive(Debug, Clone)]
		pub struct $unitName;

		impl BaseUnit for $unitName {
			fn long(&self) -> &str {
				$longName
			}

			fn short(&self) -> Option<&str> {
				$shortName
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
	($unitName:ident, $longName:expr) => {
		create_standard_unit!(full $unitName, $longName, None, []);
	};

	($unitName:ident, $longName:expr, $shortName:expr) => {
		create_standard_unit!(full $unitName, $longName, Some($shortName), []);
	};

	($unitName:ident, $longName:expr, $shortName:expr, [ $($otherUnitName:ty = $amount:expr),* ]) => {
		create_standard_unit!(full $unitName, $longName, Some($shortName), [ $($otherUnitName:ty = $amount:expr),* ]);
	};

	(full $unitName:ident, $longName:expr, $shortName:expr, [ $($otherUnitName:ty = $amount:expr),* ]) => {
		#[derive(Debug, Clone)]
		pub struct $unitName;

		impl BaseUnit for $unitName {
			fn long(&self) -> &str {
				$longName
			}

			fn short(&self) -> Option<&str> {
				$shortName
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


impl fmt::Display for dyn BaseUnit {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if let Some(short) = self.short() {
			f.write_str(short)
		} else {
			f.write_str(self.long())
		}
	}
}


pub fn is_convertable(from: &dyn BaseUnit, to: &dyn BaseUnit) -> bool {
	from.base_unit()
	.unwrap_or(from)
	.can_convert_to(to)
}