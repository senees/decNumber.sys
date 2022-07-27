use crate::dec_number::{dec_number_zero, DecNumber};

#[repr(C)]
pub struct DecimalNumber(DecNumber);

impl Default for DecimalNumber {
  /// The default value for [DecimalNumber] is zero.
  fn default() -> Self {
    let mut dn = DecNumber::default();
    dec_number_zero(&mut dn);
    Self(dn)
  }
}
