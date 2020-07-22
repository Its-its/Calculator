// https://en.wikipedia.org/wiki/Orders_of_magnitude_(data)

use crate::{create_non_standard_unit, create_standard_unit};
use super::BaseUnit;


create_standard_unit!(Byte, "byte", "bytes", "B");

create_non_standard_unit!(Bit, Byte, 0.125, "bit", "bits", "bit");
create_non_standard_unit!(KiloByte, Byte, 1024.0, "kilobyte", "kilobytes", "kB");
create_non_standard_unit!(MegaByte, Byte, 1024.0e3, "megabyte", "megabytes", "MB");
create_non_standard_unit!(GigaByte, Byte, 1024.0e6, "gigabyte", "gigabytes", "GB");
create_non_standard_unit!(TeraByte, Byte, 1024.0e9, "terabyte", "terabytes", "TB");
create_non_standard_unit!(PetaByte, Byte, 1024.0e12, "petabyte", "petabytes", "PB");
create_non_standard_unit!(ExaByte, Byte, 1024.0e15, "exabyte", "exabytes", "EB");
create_non_standard_unit!(ZettaByte, Byte, 1024.0e18, "zettabyte", "zettabytes", "ZB");
create_non_standard_unit!(YottaByte, Byte, 1024.0e21, "yottabyte", "yottabytes", "YB");