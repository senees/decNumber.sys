//!

use crate::dec_number_c::*;
use libc::c_char;
use std::ffi::CStr;

#[repr(C)]
pub struct DecNumber {
  digits: i32,
  exponent: i32,
  bits: u8,
  lsu: [u16; 12],
}

impl DecNumber {
  pub fn new(digits: i32) -> Self {
    Self {
      digits,
      exponent: 0,
      bits: 0,
      lsu: [0; 12],
    }
  }
}

/// Converts [DecNumber] into [String].
pub fn dec_number_to_string(dn: &DecNumber) -> String {
  unsafe {
    let mut buf = Vec::<char>::with_capacity((dn.digits + 14) as usize);
    decNumberToString(dn, buf.as_mut_ptr() as *mut c_char);
    CStr::from_ptr(buf.as_ptr() as *const c_char)
      .to_string_lossy()
      .into_owned()
  }
}

/// Sets [DecNumber] to zero.
pub fn dec_number_zero(dn: &mut DecNumber) {
  unsafe {
    decNumberZero(dn);
  }
}
