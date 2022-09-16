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

//! Safe bindings for 32-bit decimal.

use crate::dec_context::DecContext;
use crate::dec_single_c::*;
use libc::c_char;
use std::ffi::{CStr, CString};
use std::fmt::Debug;

/// Length in bytes of [DecSingle] union.
pub const DEC_SINGLE_BYTES: usize = 4;

/// Maximum length of the [DecSingle] string.
pub const DEC_SINGLE_STRING: usize = 16;

/// Buffer for [DecSingle] string.
pub const DEC_SINGLE_STRING_BUFFER: [c_char; DEC_SINGLE_STRING] = [0; DEC_SINGLE_STRING];

/// Convenient constant for [DecSingle] equal to positive zero.
#[rustfmt::skip]
pub const DEC_SINGLE_ZERO: DecSingle = {
  #[cfg(target_endian = "little")]
  { DecSingle { bytes: [0x00, 0x00, 0x50, 0x22] }}
  #[cfg(target_endian = "big")]
  { DecSingle { bytes: [0x22, 0x50, 0x00, 0x00] }}
};

/// 32-bit decimal number.
#[repr(C)]
#[derive(Copy, Clone)]
pub union DecSingle {
  pub bytes: [u8; DEC_SINGLE_BYTES],
  pub shorts: [u16; DEC_SINGLE_BYTES / 2],
  pub words: [u32; DEC_SINGLE_BYTES / 4],
}

impl Default for DecSingle {
  /// The default value for [DecSingle] is positive zero.
  fn default() -> Self {
    DEC_SINGLE_ZERO
  }
}

impl Debug for DecSingle {
  /// Converts [DecSingle] to a string in the form of hexadecimal bytes separated with spaces.
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "[{}]",
      (0..DEC_SINGLE_BYTES)
        .into_iter()
        .rev()
        .map(|i| format!(" {:02X}", unsafe { self.bytes[i] }))
        .collect::<String>()
        .trim_start()
    )
  }
}

/// Safe binding to *decSingleFromString* function.
pub fn dec_single_from_string(s: &str, dc: &mut DecContext) -> DecSingle {
  let c_s = CString::new(s).unwrap();
  let mut ds = DEC_SINGLE_ZERO;
  unsafe {
    decSingleFromString(&mut ds, c_s.as_ptr(), dc);
  }
  ds
}

/// Safe binding to *decSingleToString* function.
pub fn dec_single_to_string(ds1: &DecSingle) -> String {
  unsafe {
    let mut buf = DEC_SINGLE_STRING_BUFFER;
    decSingleToString(ds1, buf.as_mut_ptr() as *mut c_char);
    CStr::from_ptr(buf.as_ptr() as *const c_char)
      .to_string_lossy()
      .into_owned()
  }
}

/// Safe binding to *decSingleZero* function.
pub fn dec_single_zero(ds1: &mut DecSingle) {
  unsafe {
    decSingleZero(ds1);
  }
}
