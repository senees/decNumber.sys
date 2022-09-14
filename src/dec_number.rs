/*
 * MIT License
 *
 * Copyright (c) 2022 Dariusz Depta Engos Software
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

//! Arbitrary precision decimal definitions.

use crate::dec_number_c::*;
use crate::DecContext;
use libc::{c_char, c_uchar};
use std::ffi::{CStr, CString};

/// Sign: 1=negative, 0=positive or zero.
pub const DEC_NEG: u8 = 0x80;

/// 1 = Infinity
pub const DEC_INF: u8 = 0x40;

/// 1 = NaN
pub const DEC_NAN: u8 = 0x20;

/// 1 = sNaN
pub const DEC_SNAN: u8 = 0x10;

/// Any special value.
pub const DEC_SPECIAL: u8 = DEC_INF | DEC_NAN | DEC_SNAN;

/// Arbitrary precision decimal type.
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
  pub fn two() -> Self {
    DecNumber {
      digits: 1,
      exponent: 0,
      bits: 0,
      lsu: [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
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
pub fn dec_number_add(dn1: &DecNumber, dn2: &DecNumber, dc: &mut DecContext) -> DecNumber {
  let mut res = DecNumber::default();
  unsafe {
    decNumberAdd(&mut res, dn1, dn2, dc);
  }
  res
}

///
pub fn dec_number_compare(dn1: &DecNumber, dn2: &DecNumber, dc: &mut DecContext) -> DecNumber {
  let mut res = DecNumber::default();
  unsafe {
    decNumberCompare(&mut res, dn1, dn2, dc);
  }
  res
}

///
pub fn dec_number_divide(dn1: &DecNumber, dn2: &DecNumber, dc: &mut DecContext) -> DecNumber {
  let mut res = DecNumber::default();
  unsafe {
    decNumberDivide(&mut res, dn1, dn2, dc);
  }
  res
}

///
pub fn dec_number_exp(dn: &DecNumber, dc: &mut DecContext) -> DecNumber {
  let mut res = DecNumber::default();
  unsafe {
    decNumberExp(&mut res, dn, dc);
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
pub fn dec_number_from_i64(n: i64, dc: &mut DecContext) -> DecNumber {
  let mut res = DecNumber::default();
  let digits = bcd_digits(n.unsigned_abs() as u128);
  let count = digits.len();
  res.digits = count as i32;
  unsafe {
    decNumberSetBCD(&mut res, digits.as_ptr() as *const c_uchar, count as u32);
    if n < 0 {
      decNumberMinus(&mut res, &res, dc);
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
pub fn dec_number_from_i128(n: i128, dc: &mut DecContext) -> DecNumber {
  let mut res = DecNumber::default();
  let digits = bcd_digits(n.unsigned_abs());
  let count = digits.len();
  res.digits = count as i32;
  unsafe {
    decNumberSetBCD(&mut res, digits.as_ptr() as *const c_uchar, count as u32);
    if n < 0 {
      decNumberMinus(&mut res, &res, dc);
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
pub fn dec_number_from_isize(n: isize, dc: &mut DecContext) -> DecNumber {
  let mut res = DecNumber::default();
  let digits = bcd_digits(n.unsigned_abs() as u128);
  let count = digits.len();
  res.digits = count as i32;
  unsafe {
    decNumberSetBCD(&mut res, digits.as_ptr() as *const c_uchar, count as u32);
    if n < 0 {
      decNumberMinus(&mut res, &res, dc);
    }
  }
  res
}

/// Converts [DecNumber]  from string.
pub fn dec_number_from_string(s: &str, dc: &mut DecContext) -> DecNumber {
  let c_s = CString::new(s).unwrap();
  let mut value = DecNumber::default();
  unsafe {
    decNumberFromString(&mut value, c_s.as_ptr(), dc);
  }
  value
}

///
pub fn dec_number_ln(dn: &DecNumber, dc: &mut DecContext) -> DecNumber {
  let mut res = DecNumber::default();
  unsafe {
    decNumberLn(&mut res, dn, dc);
  }
  res
}

///
pub fn dec_number_minus(dn: &DecNumber, dc: &mut DecContext) -> DecNumber {
  let mut res = DecNumber::default();
  unsafe {
    decNumberMinus(&mut res, dn, dc);
  }
  res
}

///
pub fn dec_number_multiply(dn1: &DecNumber, dn2: &DecNumber, dc: &mut DecContext) -> DecNumber {
  let mut res = DecNumber::default();
  unsafe {
    decNumberMultiply(&mut res, dn1, dn2, dc);
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
pub fn dec_number_reduce(dn: &DecNumber, dc: &mut DecContext) -> DecNumber {
  let mut res = DecNumber::default();
  unsafe {
    decNumberReduce(&mut res, dn, dc);
  }
  res
}

///
pub fn dec_number_rescale(dn1: &DecNumber, dn2: &DecNumber, dc: &mut DecContext) -> DecNumber {
  let mut res = DecNumber::default();
  unsafe {
    decNumberRescale(&mut res, dn1, dn2, dc);
  }
  res
}

///
pub fn dec_number_scale_b(dn1: &DecNumber, dn2: &DecNumber, dc: &mut DecContext) -> DecNumber {
  let mut res = DecNumber::default();
  unsafe {
    decNumberScaleB(&mut res, dn1, dn2, dc);
  }
  res
}

///
pub fn dec_number_subtract(dn1: &DecNumber, dn2: &DecNumber, dc: &mut DecContext) -> DecNumber {
  let mut res = DecNumber::default();
  unsafe {
    decNumberSubtract(&mut res, dn1, dn2, dc);
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
