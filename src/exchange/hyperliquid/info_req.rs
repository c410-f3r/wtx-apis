#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub(crate) enum InfoReq<'any> {
  OrderStatus { user: &'any str, oid: u64 },
}
