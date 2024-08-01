use wtx::misc::{ArrayString, Lease};

pub type PhoneOwned = Phone<ArrayString<4>, ArrayString<8>>;
pub type PhoneRef<'any> = Phone<&'any str, &'any str>;

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Phone<DD, N>
where
  DD: Lease<str>,
  N: Lease<str>,
{
  pub ddd: DD,
  pub ddi: DD,
  pub number: N,
}
