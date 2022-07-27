use crate::dec_number::{dec_number_to_string, dec_number_zero, DecNumber};

#[repr(C)]
pub struct DecimalNumber(DecNumber);

impl Default for DecimalNumber {
  /// The default value for [DecimalNumber] is zero.
  fn default() -> Self {
    let mut dn = DecNumber::new(34);
    dec_number_zero(&mut dn);
    Self(dn)
  }
}

impl std::fmt::Display for DecimalNumber {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", dec_number_to_string(&self.0))
  }
}
