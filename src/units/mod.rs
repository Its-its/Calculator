// https://en.wikipedia.org/wiki/Metric_prefix
// https://en.wikipedia.org/wiki/Non-SI_units_mentioned_in_the_SI

use std::fmt;
use std::cmp;

use crate::Units;

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
	($unitName:ident, $baseUnit:expr, $factor:expr, $longName:expr, $multiName:expr) => {
		create_non_standard_unit!(full $unitName, $baseUnit, $factor, $longName, $multiName, None, []);
	};

	($unitName:ident, $baseUnit:expr, $factor:expr, $longName:expr, $multiName:expr, $shortName:expr) => {
		create_non_standard_unit!(full $unitName, $baseUnit, $factor, $longName, $multiName, Some($shortName), []);
	};

	($unitName:ident, $baseUnit:expr, $factor:expr, $longName:expr, $multiName:expr, $shortName:expr, [$($alts:expr),*]) => {
		create_non_standard_unit!(full $unitName, $baseUnit, $factor, $longName, $multiName, Some($shortName), [$($alts),*]);
	};


	(full $unitName:ident, $baseUnit:expr, $factor:expr, $longName:expr, $multiName:expr, $shortName:expr, [$($alts:expr),*]) => {
		#[derive(Debug, Clone)]
		pub struct $unitName;

		impl BaseUnit for $unitName {
			fn multiple(&self) -> &str {
				$multiName
			}

			fn long(&self) -> &str {
				$longName
			}

			fn short(&self) -> Option<&str> {
				$shortName
			}

			fn alt(&self) -> Vec<&str> {
				vec![$($alts),*]
			}

			fn base_factor(&self) -> f64 {
				$factor
			}

			fn base_unit(&self) -> &dyn BaseUnit {
				$baseUnit.base_unit()
			}
		}
	};
}

#[macro_export]
macro_rules! create_standard_unit {
	($unitName:ident, $longName:expr, $multiName:expr) => {
		create_standard_unit!(full $unitName, $longName, $multiName, None, []);
	};

	($unitName:ident, $longName:expr, $multiName:expr, $shortName:expr) => {
		create_standard_unit!(full $unitName, $longName, $multiName, Some($shortName), []);
	};

	($unitName:ident, $longName:expr, $multiName:expr, $shortName:expr, [$($alts:expr),*]) => {
		create_standard_unit!(full $unitName, $longName, $longName, $multiName, Some($shortName), [$($alts),*]);
	};

	(full $unitName:ident, $longName:expr, $multiName:expr, $shortName:expr, [$($alts:expr),*]) => {
		#[derive(Debug, Clone)]
		pub struct $unitName;

		impl BaseUnit for $unitName {
			fn multiple(&self) -> &str {
				$multiName
			}

			fn long(&self) -> &str {
				$longName
			}

			fn short(&self) -> Option<&str> {
				$shortName
			}

			fn alt(&self) -> Vec<&str> {
				vec![$($alts),*]
			}

			fn base_unit(&self) -> &dyn BaseUnit {
				self
			}
		}
	};
}


// Adds the ability to clone Box<dyn BaseUnit>
pub trait CloneBaseUnit {
	fn clone_base_unit(&self) -> Box<dyn BaseUnit>;
}

impl<T> CloneBaseUnit for T where T: BaseUnit + Clone + 'static {
	fn clone_base_unit(&self) -> Box<dyn BaseUnit> {
		Box::new(self.clone())
	}
}

impl Clone for Box<dyn BaseUnit> {
	fn clone(&self) -> Self {
		self.clone_base_unit()
	}
}


pub trait BaseUnit: fmt::Debug + CloneBaseUnit {
	/// Display name for multiple units.
	fn multiple(&self) -> &str;
	/// Display name for singular unit.
	fn long(&self) -> &str;

	fn short(&self) -> Option<&str>;
	fn alt(&self) -> Vec<&str>;

	fn base_factor(&self) -> f64 {
		1.0
	}

	fn base_unit(&self) -> &dyn BaseUnit;

	fn base_long(&self) -> &str {
		self.base_unit().long()
	}
}

// Simple check to see if the base long name == other long name.
impl PartialEq for dyn BaseUnit {
	fn eq(&self, other: &dyn BaseUnit) -> bool {
		self.base_long() == other.base_long()
	}
}

impl PartialOrd for dyn BaseUnit {
	fn partial_cmp(&self, other: &dyn BaseUnit) -> Option<cmp::Ordering> {
		Some(
			if self.base_factor() > other.base_factor() {
				cmp::Ordering::Greater
			} else if self.base_factor() > other.base_factor() {
				cmp::Ordering::Less
			} else {
				cmp::Ordering::Equal
			}
		)
	}
}

impl Ord for dyn BaseUnit {
	fn cmp(&self, other: &dyn BaseUnit) -> cmp::Ordering {
		if self.base_factor() > other.base_factor() {
			cmp::Ordering::Greater
		} else if self.base_factor() > other.base_factor() {
			cmp::Ordering::Less
		} else {
			cmp::Ordering::Equal
		}
	}
}

impl Eq for dyn BaseUnit {}

impl fmt::Display for dyn BaseUnit {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if let Some(short) = self.short() {
			f.write_str(short)
		} else {
			f.write_str(self.long())
		}
	}
}


pub fn is_convertable(from: &Units, to: &Units) -> bool {
	let from_base = from.base();
	let to_base = to.base();

	if from_base != to_base {
		return false;
	}

	let from_base = from.base_2();
	let to_base = to.base_2();

	match (from_base, to_base) {
		(Some(from), Some(to)) => from == to,
		_ => true
	}
}