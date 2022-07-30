use crate::dec_number::{dec_number_to_string, DecNumber};
use crate::ContextKind::Base;
use crate::{dec_context_default, dec_number_from_string, DecContext};

#[repr(C)]
pub struct DecimalNumber(DecNumber);

impl Default for DecimalNumber {
  /// The default value of [DecimalNumber] is zero.
  fn default() -> Self {
    Self(DecNumber::default())
  }
}

impl std::fmt::Display for DecimalNumber {
  /// Converts [DecimalNumber] into string.
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", dec_number_to_string(&self.0))
  }
}

impl From<&str> for DecimalNumber {
  ///
  fn from(s: &str) -> Self {
    let mut ctx = Self::default_context();
    let dn = dec_number_from_string(s, &mut ctx);
    Self(dn)
  }
}

impl DecimalNumber {
  ///
  fn default_context() -> DecContext {
    dec_context_default(Base(34))
  }
  ///
  pub fn zero() -> Self {
    Self(DecNumber::zero())
  }
  ///
  pub fn one() -> Self {
    Self(DecNumber::one())
  }
  ///
  pub fn ten() -> Self {
    Self(DecNumber::ten())
  }
  ///
  pub fn one_hundred() -> Self {
    Self(DecNumber::one_hundred())
  }
  ///
  pub fn one_thousand() -> Self {
    Self(DecNumber::one_thousand())
  }
}
