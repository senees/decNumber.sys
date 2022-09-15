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

/// Length in bytes of [DecDouble] union.
pub const DEC_DOUBLE_BYTES: usize = 8;

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

/// Safe binding to *decDoubleAdd* function.
pub fn dec_double_add(lhs: &DecDouble, rhs: &DecDouble, dc: &mut DecContext) -> DecDouble {
  let mut res = DecDouble::default();
  unsafe {
    decDoubleAdd(&mut res, lhs, rhs, dc);
  }
  res
}

/// Safe binding to *decDoubleZero* function.
pub fn dec_double_zero(res: &mut DecDouble) {
  unsafe {
    decDoubleZero(res);
  }
}
