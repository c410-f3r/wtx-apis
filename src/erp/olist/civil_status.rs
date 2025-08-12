/// Civil status
#[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum CivilStatus {
  /// Married
  Married = 1,
  /// Single
  Single = 2,
  /// Widowed
  Widowed = 3,
  /// Separated
  Separated = 4,
  /// Divorced
  Divorced = 5,
}
