use crate::create_non_standard_unit;
use super::{BaseUnit, Second};


create_non_standard_unit!(NanoSecond, Second, 1e-9, "nanosecond", "nanoseconds", "ns");
create_non_standard_unit!(MicroSecond, Second, 1e-6, "microsecond", "microseconds", "μs");
create_non_standard_unit!(MilliSecond, Second, 1e-3, "millisecond", "milliseconds", "ms");
create_non_standard_unit!(Minute, Second, 60.0, "minute", "minutes", "min");
create_non_standard_unit!(Hour, Second, 3600.0, "hour", "hours", "h");
create_non_standard_unit!(Day, Second, 3600.0 * 24.0, "day", "days", "d");
create_non_standard_unit!(Week, Second, 3600.0 * 24.0 * 7.0, "week", "weeks", "w");
create_non_standard_unit!(Fortnight, Second, 3600.0 * 24.0 * 14.0, "fortnight", "fortnights");
create_non_standard_unit!(Month, Second, (30.0 * 24.0 + 10.5) * 3600.0, "month", "months");
create_non_standard_unit!(Year, Second, 3600.0 * 24.0 * 365.0, "year", "years");
create_non_standard_unit!(CommonYear, Second, 365.25 * 24.0 * 3600.0, "common year", "common years", "cy");
create_non_standard_unit!(Decade, Second, 3600.0 * 24.0 * 365.0 * 10.0, "decade", "decades");
create_non_standard_unit!(Century, Second, 3600.0 * 24.0 * 365.0 * 100.0, "century", "centuries");