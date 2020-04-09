use crate::create_non_standard_unit;
use super::{BaseUnit, Second};


create_non_standard_unit!(NanoSecond, Second, 1e-9, "nanosecond", "ns");
create_non_standard_unit!(MicroSecond, Second, 1e-6, "microsecond", "μs");
create_non_standard_unit!(MilliSecond, Second, 1e-3, "millisecond", "ms");
create_non_standard_unit!(Minute, Second, 60.0, "minute", "min");
create_non_standard_unit!(Hour, Second, 3600.0, "hour", "h");
create_non_standard_unit!(Day, Second, 3600.0 * 24.0, "day", "d");
create_non_standard_unit!(Week, Second, 3600.0 * 24.0 * 7.0, "week", "w");
create_non_standard_unit!(Fortnight, Second, 3600.0 * 24.0 * 14.0, "fortnight");
create_non_standard_unit!(Month, Second, (30.0 * 24.0 + 10.5) * 3600.0, "month");
create_non_standard_unit!(Year, Second, 3600.0 * 24.0 * 365.0, "year");
create_non_standard_unit!(CommonYear, Second, 365.25 * 24.0 * 3600.0, "common year", "cy");
create_non_standard_unit!(Decade, Second, 3600.0 * 24.0 * 365.0 * 10.0, "decade");
create_non_standard_unit!(Century, Second, 3600.0 * 24.0 * 365.0 * 100.0, "century");