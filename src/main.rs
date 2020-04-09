
pub mod error;
pub mod units;
pub mod quantity;

pub use error::{Error, Result};
pub use units::*;
pub use quantity::*;

fn main() {
	// test_convert_number(1.0, &MONTH, &DAY);

	// println!("Year -> ?");
	// test_convert_number(1.0, &YEAR, &MONTH);
	// test_convert_number(1.0, &YEAR, &WEEK);
	// test_convert_number(1.0, &YEAR, &DAY);
	// test_convert_number(1.0, &YEAR, &HOUR);
	// test_convert_number(1.0, &YEAR, &MINUTE);

	// println!("Month -> ?");
	// test_convert_number(1.0, &MONTH, &YEAR);
	// test_convert_number(1.0, &MONTH, &WEEK);
	// test_convert_number(1.0, &MONTH, &DAY);
	// test_convert_number(1.0, &MONTH, &HOUR);
	// test_convert_number(1.0, &MONTH, &MINUTE);

	// println!("Week -> ?");
	// test_convert_number(1.0, &WEEK, &YEAR);
	// test_convert_number(1.0, &WEEK, &MONTH);
	// test_convert_number(1.0, &WEEK, &DAY);
	// test_convert_number(1.0, &WEEK, &HOUR);
	// test_convert_number(1.0, &WEEK, &MINUTE);

	// println!("Day -> ?");
	// test_convert_number(1.0, &DAY, &YEAR);
	// test_convert_number(1.0, &DAY, &MONTH);
	// test_convert_number(1.0, &DAY, &WEEK);
	// test_convert_number(1.0, &DAY, &HOUR);
	// test_convert_number(1.0, &DAY, &MINUTE);

	// println!("Hour -> ?");
	// test_convert_number(1.0, &HOUR, &YEAR);
	// test_convert_number(1.0, &HOUR, &MONTH);
	// test_convert_number(1.0, &HOUR, &DAY);
	// test_convert_number(1.0, &HOUR, &WEEK);
	// test_convert_number(1.0, &HOUR, &MINUTE);

	// println!("Minute -> ?");
	// test_convert_number(1.0, &MINUTE, &YEAR);
	// test_convert_number(1.0, &MINUTE, &MONTH);
	// test_convert_number(1.0, &MINUTE, &DAY);
	// test_convert_number(1.0, &MINUTE, &HOUR);
	// test_convert_number(1.0, &MINUTE, &WEEK);

	// println!("? -> Second");
	// test_convert_number(1.0, &YEAR, &SECOND);
	// test_convert_number(1.0, &MONTH, &SECOND);
	// test_convert_number(1.0, &DAY, &SECOND);
	// test_convert_number(1.0, &HOUR, &SECOND);
	// test_convert_number(1.0, &MINUTE, &SECOND);
}

// pub fn test_convert_number<V: Into<units::Value>>(quantity: V, from: &units::BaseUnit, to: &units::BaseUnit) {
// 	let quantity = quantity.into();


// 	match units::convert(&quantity, &from, &to) {
// 		Ok(converted) => {
// 			println!(
// 				"{:?} {} -> {} = {:?} {}(s)",
// 				quantity.as_number(),
// 				from.long(),
// 				to.long(),
// 				converted,
// 				to.long()
// 			);
// 		}

// 		Err(err) => {
// 			eprintln!("Conversion Error: {}", err);
// 		}
// 	}
// }