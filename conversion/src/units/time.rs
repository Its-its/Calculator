use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::create_non_standard_unit;
use super::{BaseUnit, Second};


create_non_standard_unit!(NanoSecond, Second, dec!(1e-9), "nanosecond", "nanoseconds", "ns");
create_non_standard_unit!(MicroSecond, Second, dec!(1e-6), "microsecond", "microseconds", "μs");
create_non_standard_unit!(MilliSecond, Second, dec!(1e-3), "millisecond", "milliseconds", "ms");
create_non_standard_unit!(Minute, Second, dec!(60.0), "minute", "minutes", "min");
create_non_standard_unit!(Hour, Second, dec!(3600.0), "hour", "hours", "h");
create_non_standard_unit!(Day, Second, Decimal::new(3600 * 24, 0), "day", "days", "d");
create_non_standard_unit!(Week, Second, Decimal::new(3600 * 24 * 7, 0), "week", "weeks", "w");
create_non_standard_unit!(Fortnight, Second, Decimal::new(3600 * 24 * 14, 0), "fortnight", "fortnights");
create_non_standard_unit!(Month, Second, Decimal::new(2629800, 0), "month", "months"); // (30 * 24 + 10.5) * 3600
create_non_standard_unit!(Year, Second, Decimal::new(3600 * 24 * 365, 0), "year", "years");
create_non_standard_unit!(CommonYear, Second, Decimal::new(31557600, 0), "common year", "common years", "cy"); // 365.25 * 24.0 * 3600.0
create_non_standard_unit!(Decade, Second, Decimal::new(3600 * 24 * 365 * 10, 0), "decade", "decades");
create_non_standard_unit!(Century, Second, Decimal::new(3600 * 24 * 365 * 100, 0), "century", "centuries");