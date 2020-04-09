// https://en.wikipedia.org/wiki/Orders_of_magnitude_(data)

use crate::{create_non_standard_unit, create_standard_unit};
use super::BaseUnit;


create_standard_unit!(Byte, "byte", "B");

create_non_standard_unit!(Bit, Byte, 0.125, "bit", "bit");
create_non_standard_unit!(KiloByte, Byte, 1024.0, "kilobyte", "kB");
create_non_standard_unit!(MegaByte, Byte, 1024.0e3, "megabyte", "MB");
create_non_standard_unit!(GigaByte, Byte, 1024.0e6, "gigabyte", "GB");
create_non_standard_unit!(TeraByte, Byte, 1024.0e9, "terabyte", "TB");
create_non_standard_unit!(PetaByte, Byte, 1024.0e12, "petabyte", "PB");
create_non_standard_unit!(ExaByte, Byte, 1024.0e15, "exabyte", "EB");
create_non_standard_unit!(ZettaByte, Byte, 1024.0e18, "zettabyte", "ZB");
create_non_standard_unit!(YottaByte, Byte, 1024.0e21, "yottabyte", "YB");