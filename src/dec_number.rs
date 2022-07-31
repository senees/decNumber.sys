//!

use crate::dec_number_c::*;
use crate::DecContext;
use libc::{c_char, c_uchar};
use std::ffi::{CStr, CString};

/// Sign; 1=negative, 0=positive or zero.
const DEC_NEG: u8 = 0x80;
/// 1 = Infinity
const DEC_INF: u8 = 0x40;
/// 1 = NaN
const DEC_NAN: u8 = 0x20;
/// 1 = sNaN
const DEC_SNAN: u8 = 0x10;
/// Any special value.
const DEC_SPECIAL: u8 = DEC_INF | DEC_NAN | DEC_SNAN;

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
pub fn dec_number_compare(dn1: &DecNumber, dn2: &DecNumber, ctx: &mut DecContext) -> DecNumber {
  let mut res = DecNumber::default();
  unsafe {
    decNumberCompare(&mut res, dn1, dn2, ctx);
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

///
pub fn dec_number_exp(dn: &DecNumber, ctx: &mut DecContext) -> DecNumber {
  let mut res = DecNumber::default();
  unsafe {
    decNumberExp(&mut res, dn, ctx);
  }
  res
}

///
pub fn dec_number_from_i32(n: i32) -> DecNumber {
  let mut res = DecNumber::default();
  unsafe {
    decNumberFromInt32(&mut res, n);
  }
  res
}

///
pub fn dec_number_from_u32(n: u32) -> DecNumber {
  let mut result = DecNumber::default();
  unsafe {
    decNumberFromUInt32(&mut result, n);
  }
  result
}

///
pub fn dec_number_from_u64(n: u64) -> DecNumber {
  let mut res = DecNumber::default();
  let digits = bcd_digits(n as u128);
  let count = digits.len();
  res.digits = count as i32;
  unsafe {
    decNumberSetBCD(&mut res, digits.as_ptr() as *const c_uchar, count as u32);
  }
  res
}

///
pub fn dec_number_from_i64(n: i64, ctx: &mut DecContext) -> DecNumber {
  let mut res = DecNumber::default();
  let digits = bcd_digits(n.unsigned_abs() as u128);
  let count = digits.len();
  res.digits = count as i32;
  unsafe {
    decNumberSetBCD(&mut res, digits.as_ptr() as *const c_uchar, count as u32);
    if n < 0 {
      decNumberMinus(&mut res, &res, ctx);
    }
  }
  res
}

///
pub fn dec_number_from_u128(n: u128) -> DecNumber {
  let mut res = DecNumber::default();
  let digits = bcd_digits(n);
  let count = digits.len();
  res.digits = count as i32;
  unsafe {
    decNumberSetBCD(&mut res, digits.as_ptr() as *const c_uchar, count as u32);
  }
  res
}

///
pub fn dec_number_from_i128(n: i128, ctx: &mut DecContext) -> DecNumber {
  let mut res = DecNumber::default();
  let digits = bcd_digits(n.unsigned_abs());
  let count = digits.len();
  res.digits = count as i32;
  unsafe {
    decNumberSetBCD(&mut res, digits.as_ptr() as *const c_uchar, count as u32);
    if n < 0 {
      decNumberMinus(&mut res, &res, ctx);
    }
  }
  res
}

///
pub fn dec_number_from_usize(n: usize) -> DecNumber {
  let mut res = DecNumber::default();
  let digits = bcd_digits(n as u128);
  let count = digits.len();
  res.digits = count as i32;
  unsafe {
    decNumberSetBCD(&mut res, digits.as_ptr() as *const c_uchar, count as u32);
  }
  res
}

///
pub fn dec_number_from_isize(n: isize, ctx: &mut DecContext) -> DecNumber {
  let mut res = DecNumber::default();
  let digits = bcd_digits(n.unsigned_abs() as u128);
  let count = digits.len();
  res.digits = count as i32;
  unsafe {
    decNumberSetBCD(&mut res, digits.as_ptr() as *const c_uchar, count as u32);
    if n < 0 {
      decNumberMinus(&mut res, &res, ctx);
    }
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
pub fn dec_number_ln(dn: &DecNumber, ctx: &mut DecContext) -> DecNumber {
  let mut res = DecNumber::default();
  unsafe {
    decNumberLn(&mut res, dn, ctx);
  }
  res
}

///
pub fn dec_number_minus(dn: &DecNumber, ctx: &mut DecContext) -> DecNumber {
  let mut res = DecNumber::default();
  unsafe {
    decNumberMinus(&mut res, dn, ctx);
  }
  res
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
pub fn dec_number_is_negative(dn: &DecNumber) -> bool {
  dn.bits & DEC_NEG != 0
}

///
pub fn dec_number_is_zero(dn: &DecNumber) -> bool {
  dn.lsu[0] == 0 && dn.digits == 1 && (dn.bits & DEC_SPECIAL == 0)
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
pub fn dec_number_rescale(dn1: &DecNumber, dn2: &DecNumber, ctx: &mut DecContext) -> DecNumber {
  let mut res = DecNumber::default();
  unsafe {
    decNumberRescale(&mut res, dn1, dn2, ctx);
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

///
fn bcd_digits(n: u128) -> Vec<u8> {
  let mut v = n;
  let mut digits = Vec::<u8>::with_capacity(20);
  loop {
    digits.push((v % 10) as u8);
    v /= 10;
    if v == 0 {
      break;
    }
  }
  digits.reverse();
  digits
}
