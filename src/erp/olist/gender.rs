use serde::{Serialize, Serializer};

wtx::create_enum! {
  /// Gender
  #[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize)]
  #[serde(try_from = "&str")]
  pub enum Gender<u8> {
    /// Female
    Female = (1, "Feminino"),
    /// Male
    Male = (2, "Masculino"),
  }
}

impl Serialize for Gender {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.strings().custom[0])
  }
}
