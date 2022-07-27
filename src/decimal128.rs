//! Implementation of `DECIMAL128`.

use crate::dec_quad::DecQuad;
use crate::{dec_context_default, dec_quad_from_string, dec_quad_to_string, ContextKind, DecContext};

#[repr(C)]
pub struct Decimal128(DecQuad, DecContext);

impl Default for Decimal128 {
  /// The default value for [Decimal128] is zero.
  fn default() -> Self {
    Self(DecQuad::default(), dec_context_default(ContextKind::Decimal128))
  }
}

impl std::fmt::Display for Decimal128 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", dec_quad_to_string(&self.0))
  }
}

impl std::fmt::Debug for Decimal128 {
  /// Converts [Decimal128] to a string in the form of hexadecimal bytes separated with spaces.
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self.0)
  }
}

impl From<&str> for Decimal128 {
  /// Converts [Decimal128] from string.
  fn from(s: &str) -> Self {
    let mut dec_context = dec_context_default(ContextKind::Decimal128);
    let dec_quad = dec_quad_from_string(s, &mut dec_context);
    Self(dec_quad, dec_context)
  }
}
