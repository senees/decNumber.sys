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

// /// Adds two [DecSingles](DecSingle).
// pub fn dec_single_add(lhs: &DecSingle, rhs: &DecSingle, dc: &mut DecContext) -> DecSingle {
//   let mut result = DecSingle::default();
//   let mut dl = DecDouble::default();
//   let mut dr = DecDouble::default();
//   let mut d_res = DecDouble::default();
//   unsafe {
//     decSingleToWider(lhs, &mut dl, dc);
//     decSingleToWider(rhs, &mut dr, dc);
//     decSingleAdd(&mut result, lhs, rhs, dc);
//   }
//   result
// }

// /// Converts [DecSingle] from signed integer.
// pub fn dec_single_from_i32(n: i32) -> DecSingle {
//
//   let mut result = DecSingle::default();
//   let mut dq = DecDouble::default();
//   unsafe {
//     decSingleFromInt32(&mut result, n);
//   }
//   result
// }
//
// /// Converts [DecSingle] from unsigned integer.
// pub fn dec_single_from_u32(n: u32) -> DecSingle {
//   let mut result = DecSingle::default();
//   unsafe {
//     decSingleFromUInt32(&mut result, n);
//   }
//   result
// }

/// Safe binding to *decSingleFromString* function.
pub fn dec_single_from_string(s: &str, dc: &mut DecContext) -> DecSingle {
  let c_s = CString::new(s).unwrap();
  let mut ds_res = DecSingle::default();
  unsafe {
    decSingleFromString(&mut ds_res, c_s.as_ptr(), dc);
  }
  ds_res
}

/// Safe binding to *decSingleToString* function.
pub fn dec_single_to_string(ds: &DecSingle) -> String {
  unsafe {
    let mut buf = DEC_SINGLE_STRING_BUFFER;
    decSingleToString(ds, buf.as_mut_ptr() as *mut c_char);
    CStr::from_ptr(buf.as_ptr() as *const c_char)
      .to_string_lossy()
      .into_owned()
  }
}

/// Safe binding to *decSingleZero* function.
pub fn dec_single_zero(ds: &mut DecSingle) {
  unsafe {
    decSingleZero(ds);
  }
}
