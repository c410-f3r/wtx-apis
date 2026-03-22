//! Access to various system variables.

use crate::misc::HashArray32Static;

/// Public address of the program
pub const ID: HashArray32Static = HashArray32Static::new(
  [
    6, 167, 213, 23, 24, 123, 209, 102, 53, 218, 212, 4, 85, 253, 194, 192, 193, 36, 198, 143, 33,
    86, 117, 165, 219, 186, 203, 95, 8, 0, 0, 0,
  ],
  "Sysvar1nstructions1111111111111111111111111",
);
