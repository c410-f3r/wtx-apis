/// Represents the minimum and maximum possible byte lengths of a data structure.
#[derive(Debug)]
pub struct LenBounds {
  min: usize,
  max: Option<usize>,
}

impl LenBounds {
  /// Creates a new instance with the specified minimum and optional maximum lengths.
  pub const fn new(min: usize, max: Option<usize>) -> Self {
    Self { min, max }
  }

  /// Creates a new instance where the minimum and maximum lengths are identical.
  pub const fn from_same(n: usize) -> Self {
    Self { min: n, max: Some(n) }
  }

  /// Returns the maximum possible length, if defined.
  pub const fn max(&self) -> Option<usize> {
    self.max
  }

  /// Returns the maximum length if available, otherwise returns the minimum length.
  pub const fn max_or_min(&self) -> usize {
    if let Some(el) = self.max { el } else { self.min }
  }

  /// Returns the minimum possible length.
  pub const fn min(&self) -> usize {
    self.min
  }

  /// Returns a new instance with lengths added from another instance, saturating on overflow.
  #[must_use]
  pub const fn saturating_add(&self, other: &Self) -> Self {
    Self {
      min: self.min.saturating_add(other.min),
      max: match [self.max, other.max] {
        [None, None] => None,
        [None, Some(el)] | [Some(el), None] => Some(el),
        [Some(a), Some(b)] => Some(a.saturating_add(b)),
      },
    }
  }
}
