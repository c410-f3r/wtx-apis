use wtx::misc::ArrayString;

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressOwned {
  pub city: ArrayString<32>,
  pub complementary: Option<ArrayString<32>>,
  pub country: ArrayString<32>,
  pub neighborhood: ArrayString<32>,
  pub state: ArrayString<2>,
  pub street: ArrayString<32>,
  pub street_number: ArrayString<8>,
  pub zipcode: ArrayString<12>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressRef<'any> {
  pub city: &'any str,
  pub complementary: Option<&'any str>,
  pub country: &'any str,
  pub neighborhood: &'any str,
  pub state: &'any str,
  pub street_number: &'any str,
  pub street: &'any str,
  pub zipcode: &'any str,
}
