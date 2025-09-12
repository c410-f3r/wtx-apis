use wtx::calendar::{DateTime, DynTz};

/// A struct representing the payer's address information.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Address<T> {
  /// Street name.
  pub street_name: Option<T>,
  /// Street number.
  pub street_number: Option<T>,
  /// Address zip code.
  pub zip_code: Option<T>,
}

/// A struct representing the payer's identification information.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Identification<T> {
  /// The identification number.
  pub number: Option<T>,
  /// The identification type (e.g., DNI).
  pub r#type: Option<T>,
}

/// Information about the buyer.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Payer<T> {
  /// Information about the buyer's address.
  pub address: Option<Address<T>>,
  /// Buyer's registration date.
  pub date_created: Option<DateTime<DynTz>>,
  /// Buyer's email.
  pub email: Option<T>,
  /// Information about the buyer's identification.
  pub identification: Option<Identification<T>>,
  /// Buyer's first name.
  pub name: Option<T>,
  /// Information about the buyer's phone.
  pub phone: Option<Phone<T>>,
  /// Buyer's last name.
  pub surname: Option<T>,
}

/// Information about the buyer's phone.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Phone<T> {
  /// Phone area code.
  pub area_code: Option<T>,
  /// Phone number.
  pub number: Option<T>,
}
