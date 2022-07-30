//!

use crate::dec_number_c::*;
use crate::DecContext;
use libc::c_char;
use std::ffi::{CStr, CString};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DecNumber {
  digits: i32,
  exponent: i32,
  bits: u8,
  lsu: [u16; 12],
}

impl Default for DecNumber {
  /// Default value for [DecNumber] is zero.
  fn default() -> Self {
    let mut dn = Self {
      digits: 1,
      exponent: 0,
      bits: 0,
      lsu: [0; 12],
    };
    unsafe {
      decNumberZero(&mut dn);
    }
    dn
  }
}

impl DecNumber {
  ///
  pub fn zero() -> Self {
    DecNumber {
      digits: 1,
      exponent: 0,
      bits: 0,
      lsu: [0; 12],
    }
  }
  ///
  pub fn one() -> Self {
    DecNumber {
      digits: 1,
      exponent: 0,
      bits: 0,
      lsu: [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    }
  }
  ///
  pub fn ten() -> Self {
    DecNumber {
      digits: 2,
      exponent: 0,
      bits: 0,
      lsu: [10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    }
  }
  ///
  pub fn one_hundred() -> Self {
    DecNumber {
      digits: 3,
      exponent: 0,
      bits: 0,
      lsu: [100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    }
  }
  ///
  pub fn one_thousand() -> Self {
    DecNumber {
      digits: 4,
      exponent: 0,
      bits: 0,
      lsu: [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    }
  }
}

///
pub fn dec_number_add(dn1: &DecNumber, dn2: &DecNumber, ctx: &mut DecContext) -> DecNumber {
  let mut res = DecNumber::default();
  unsafe {
    decNumberAdd(&mut res, dn1, dn2, ctx);
  }
  res
}

///
pub fn dec_number_divide(dn1: &DecNumber, dn2: &DecNumber, ctx: &mut DecContext) -> DecNumber {
  let mut res = DecNumber::default();
  unsafe {
    decNumberDivide(&mut res, dn1, dn2, ctx);
  }
  res
}

/// Converts [DecNumber]  from string.
pub fn dec_number_from_string(s: &str, ctx: &mut DecContext) -> DecNumber {
  let c_s = CString::new(s).unwrap();
  let mut value = DecNumber::default();
  unsafe {
    decNumberFromString(&mut value, c_s.as_ptr(), ctx);
  }
  value
}

///
pub fn dec_number_multiply(dn1: &DecNumber, dn2: &DecNumber, ctx: &mut DecContext) -> DecNumber {
  let mut res = DecNumber::default();
  unsafe {
    decNumberMultiply(&mut res, dn1, dn2, ctx);
  }
  res
}

///
pub fn dec_number_reduce(dn: &DecNumber, ctx: &mut DecContext) -> DecNumber {
  let mut res = DecNumber::default();
  unsafe {
    decNumberReduce(&mut res, dn, ctx);
  }
  res
}

///
pub fn dec_number_subtract(dn1: &DecNumber, dn2: &DecNumber, ctx: &mut DecContext) -> DecNumber {
  let mut res = DecNumber::default();
  unsafe {
    decNumberSubtract(&mut res, dn1, dn2, ctx);
  }
  res
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
