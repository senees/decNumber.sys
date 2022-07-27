//!

use crate::dec_number_c::*;

#[repr(C)]
#[derive(Default)]
pub struct DecNumber {
  digits: i32,
  exponent: i32,
  bits: u8,
  lsu: [u16; 12],
}

/// Sets [DecNumber] to zero.
pub fn dec_number_zero(dn: &mut DecNumber) {
  unsafe {
    decNumberZero(dn);
  }
}
