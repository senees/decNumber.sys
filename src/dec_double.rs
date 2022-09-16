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

//! Safe bindings for 64-bit decimal.

use crate::dec_context::DecContext;
use crate::dec_double_c::*;
use libc::c_char;
use std::ffi::{CStr, CString};
use std::fmt::Debug;

/// Length in bytes of [DecDouble] union.
pub const DEC_DOUBLE_BYTES: usize = 8;

/// Maximum length of the [DecDouble] string.
pub const DEC_DOUBLE_STRING: usize = 25;

/// Buffer for [DecDouble] string.
pub const DEC_DOUBLE_STRING_BUFFER: [c_char; DEC_DOUBLE_STRING] = [0; DEC_DOUBLE_STRING];

/// Convenient constant for [DecDouble] equal to positive zero.
#[rustfmt::skip]
pub const DEC_DOUBLE_ZERO: DecDouble = {
  #[cfg(target_endian = "little")]
  { DecDouble { bytes: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x38, 0x22] }}
  #[cfg(target_endian = "big")]
  { DecDouble { bytes: [0x22, 0x38, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00] }}
};

/// 64-bit decimal number.
#[repr(C)]
#[derive(Copy, Clone)]
pub union DecDouble {
  pub bytes: [u8; DEC_DOUBLE_BYTES],
  pub shorts: [u16; DEC_DOUBLE_BYTES / 2],
  pub words: [u32; DEC_DOUBLE_BYTES / 4],
  pub longs: [u64; DEC_DOUBLE_BYTES / 8],
}

impl Default for DecDouble {
  /// The default value for [DecDouble] is positive zero.
  fn default() -> Self {
    DEC_DOUBLE_ZERO
  }
}

impl Debug for DecDouble {
  /// Converts [DecDouble] to a string in the form of hexadecimal bytes separated with spaces.
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "[{}]",
      (0..DEC_DOUBLE_BYTES)
        .into_iter()
        .rev()
        .map(|i| format!(" {:02X}", unsafe { self.bytes[i] }))
        .collect::<String>()
        .trim_start()
    )
  }
}

/// Safe binding to *decDoubleAdd* function.
pub fn dec_double_add(dd1: &DecDouble, dd2: &DecDouble, dc: &mut DecContext) -> DecDouble {
  let mut dd = DEC_DOUBLE_ZERO;
  unsafe {
    decDoubleAdd(&mut dd, dd1, dd2, dc);
  }
  dd
}

/// Safe binding to *decDoubleFromString* function.
pub fn dec_double_from_string(s: &str, dc: &mut DecContext) -> DecDouble {
  let c_s = CString::new(s).unwrap();
  let mut dd = DEC_DOUBLE_ZERO;
  unsafe {
    decDoubleFromString(&mut dd, c_s.as_ptr(), dc);
  }
  dd
}

/// Safe binding to *decDoubleToString* function.
pub fn dec_double_to_string(dd1: &DecDouble) -> String {
  unsafe {
    let mut buf = DEC_DOUBLE_STRING_BUFFER;
    decDoubleToString(dd1, buf.as_mut_ptr() as *mut c_char);
    CStr::from_ptr(buf.as_ptr() as *const c_char)
      .to_string_lossy()
      .into_owned()
  }
}

/// Safe binding to *decDoubleZero* function.
pub fn dec_double_zero(dd1: &mut DecDouble) {
  unsafe {
    decDoubleZero(dd1);
  }
}
