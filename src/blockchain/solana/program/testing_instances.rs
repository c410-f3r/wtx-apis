/// A trait for types that can provide predefined instances for testing purposes.
pub trait TestingInstances: Sized {
  /// Creates an instance with the minimum possible valid values.
  fn min_instance() -> crate::Result<Self>;

  /// Creates an instance with variable or typical values.
  fn variable_instance() -> crate::Result<Self> {
    Self::min_instance()
  }
}

impl<A, B> TestingInstances for (A, B)
where
  A: TestingInstances,
  B: TestingInstances,
{
  fn min_instance() -> crate::Result<Self> {
    Ok((A::min_instance()?, B::min_instance()?))
  }
}
