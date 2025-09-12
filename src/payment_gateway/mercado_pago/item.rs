use rust_decimal::Decimal;

/// Information about the item.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Item<T> {
  /// Item identifier.
  pub id: T,
  /// Item title.
  pub title: T,
  /// Item description.
  pub description: Option<T>,
  /// Item image URL.
  pub picture_url: Option<T>,
  /// Item category.
  pub category_id: Option<T>,
  /// Item quantity.
  #[serde(with = "rust_decimal::serde::float")]
  pub quantity: Decimal,
  /// Item currency.
  pub currency_id: Option<T>,
  /// Item unit price.
  #[serde(with = "rust_decimal::serde::float")]
  pub unit_price: Decimal,
}
