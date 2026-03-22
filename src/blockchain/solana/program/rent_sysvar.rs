//! Rent

use crate::misc::HashArray32Static;

/// Public address of the program
pub const ID: HashArray32Static = HashArray32Static::new(
  [
    6, 167, 213, 23, 25, 44, 92, 81, 33, 140, 201, 76, 61, 74, 241, 127, 88, 218, 238, 8, 155, 161,
    253, 68, 227, 219, 217, 138, 0, 0, 0, 0,
  ],
  "SysvarRent111111111111111111111111111111111",
);
