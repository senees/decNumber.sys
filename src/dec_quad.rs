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

//! 128-bit decimal definitions.

use crate::decContextZeroStatus;
use crate::dec_context::DecContext;
use crate::dec_quad_c::*;
use libc::c_char;
use std::ffi::{CStr, CString};
use std::fmt::Debug;

/// Length in bytes of [DecQuad] union.
pub const DEC_QUAD_BYTES: usize = 16;

/// Maximum length of the [DecQuad] string.
pub const DEC_QUAD_STRING: usize = 43;

/// Buffer for [DecQuad] string.
pub const DEC_QUAD_STRING_BUFFER: [c_char; DEC_QUAD_STRING] = [0; DEC_QUAD_STRING];

/// [DecQuad] equal to zero `0`.
#[rustfmt::skip]
pub const DEC_QUAD_ZERO: DecQuad = {
  #[cfg(target_endian = "little")]
  { DecQuad { bytes: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x22] }}
  #[cfg(target_endian = "big")]
  { DecQuad { bytes: [0x22, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00] }}
};

/// [DecQuad] equal to one `1`.
#[rustfmt::skip]
pub const DEC_QUAD_ONE: DecQuad = {
  #[cfg(target_endian = "little")]
  { DecQuad { bytes: [0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x22] }}
  #[cfg(target_endian = "big")]
  { DecQuad { bytes: [0x22, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01] }}
};

/// [DecQuad] equal to two `2`.
#[rustfmt::skip]
pub const DEC_QUAD_TWO: DecQuad = {
  #[cfg(target_endian = "little")]
  { DecQuad { bytes: [0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x22] }}
  #[cfg(target_endian = "big")]
  { DecQuad { bytes: [0x22, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02] }}
};

/// [DecQuad] equal to ten `10`.
#[rustfmt::skip]
pub const DEC_QUAD_TEN: DecQuad = {
  #[cfg(target_endian = "little")]
  { DecQuad { bytes: [0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x22] }}
  #[cfg(target_endian = "big")]
  { DecQuad { bytes: [0x22, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10] }}
};

/// [DecQuad] equal to hundred `100`.
#[rustfmt::skip]
pub const DEC_QUAD_HUNDRED: DecQuad = {
  #[cfg(target_endian = "little")]
  { DecQuad { bytes: [0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x22] }}
  #[cfg(target_endian = "big")]
  { DecQuad { bytes: [0x22, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80] }}
};

/// [DecQuad] equal to thousand `1000`.
#[rustfmt::skip]
pub const DEC_QUAD_THOUSAND: DecQuad = {
  #[cfg(target_endian = "little")]
  { DecQuad { bytes: [0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x22] }}
  #[cfg(target_endian = "big")]
  { DecQuad { bytes: [0x22, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x04, 0x00] }}
};

/// [DecQuad] equal to million `1000000`.
#[rustfmt::skip]
pub const DEC_QUAD_MILLION: DecQuad = {
  #[cfg(target_endian = "little")]
  { DecQuad { bytes: [0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x22] }}
  #[cfg(target_endian = "big")]
  { DecQuad { bytes: [0x22, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00] }}
};

/// [DecQuad] equal to billion `1000000000`.
#[rustfmt::skip]
pub const DEC_QUAD_BILLION: DecQuad = {
  #[cfg(target_endian = "little")]
  { DecQuad { bytes: [0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x22] }}
  #[cfg(target_endian = "big")]
  { DecQuad { bytes: [0x22, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00] }}
};

/// 128-bit decimal type, accessible by all sizes.
#[repr(C)]
pub union DecQuad {
  pub bytes: [u8; DEC_QUAD_BYTES],
  pub shorts: [u16; DEC_QUAD_BYTES / 2],
  pub words: [u32; DEC_QUAD_BYTES / 4],
  pub longs: [u64; DEC_QUAD_BYTES / 8],
}

impl Default for DecQuad {
  /// The default value for [DecQuad] is positive zero.
  fn default() -> Self {
    DEC_QUAD_ZERO
  }
}

impl Debug for DecQuad {
  /// Converts [DecQuad] to a string in the form of hexadecimal bytes separated with spaces.
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "[{}]",
      (0..16)
        .into_iter()
        .rev()
        .map(|i| format!(" {:02X}", unsafe { self.bytes[i] }))
        .collect::<String>()
        .trim_start()
    )
  }
}

/// Returns absolute value of [DecQuad].
pub fn dec_quad_abs(dq: &DecQuad, dc: &mut DecContext) -> DecQuad {
  let mut dq_res = DecQuad::default();
  unsafe {
    decQuadAbs(&mut dq_res, dq, dc);
  }
  dq_res
}

/// Adds two [DecQuads](DecQuad).
pub fn dec_quad_add(dq1: &DecQuad, dq2: &DecQuad, dc: &mut DecContext) -> DecQuad {
  let mut dq_res = DecQuad::default();
  unsafe {
    decQuadAdd(&mut dq_res, dq1, dq2, dc);
  }
  dq_res
}

/// Converts [DecQuad] from signed integer.
pub fn dec_quad_from_i32(n: i32) -> DecQuad {
  let mut dq_res = DecQuad::default();
  unsafe {
    decQuadFromInt32(&mut dq_res, n);
  }
  dq_res
}

/// Converts [DecQuad] from unsigned integer.
pub fn dec_quad_from_u32(n: u32) -> DecQuad {
  let mut dq_res = DecQuad::default();
  unsafe {
    decQuadFromUInt32(&mut dq_res, n);
  }
  dq_res
}

/// Converts [DecQuad] from string.
pub fn dec_quad_from_string(s: &str, dc: &mut DecContext) -> DecQuad {
  let c_s = CString::new(s).unwrap();
  let mut dq_res = DecQuad::default();
  unsafe {
    decContextZeroStatus(dc);
    decQuadFromString(&mut dq_res, c_s.as_ptr(), dc);
  }
  dq_res
}

/// Safe binding to *decQuadReduce* function.
pub fn dec_quad_reduce(dn: &DecQuad, dc: &mut DecContext) -> DecQuad {
  let mut dq_res = DecQuad::default();
  unsafe {
    decQuadReduce(&mut dq_res, dn, dc);
  }
  dq_res
}

/// Safe binding to *decQuadScaleB* function.
pub fn dec_quad_scale_b(dq1: &DecQuad, dq2: &DecQuad, dc: &mut DecContext) -> DecQuad {
  let mut dq_res = DecQuad::default();
  unsafe {
    decQuadScaleB(&mut dq_res, dq1, dq2, dc);
  }
  dq_res
}

/// Converts [DecQuad] into integral value.
pub fn dec_quad_to_integral_value(dq: &DecQuad, dc: &mut DecContext, rounding: u32) -> DecQuad {
  let mut dq_res = DecQuad::default();
  unsafe {
    decQuadToIntegralValue(&mut dq_res, dq, dc, rounding);
  }
  dq_res
}

/// Converts [DecQuad] into [String].
pub fn dec_quad_to_string(dq: &DecQuad) -> String {
  unsafe {
    let mut buf = DEC_QUAD_STRING_BUFFER;
    decQuadToString(dq, buf.as_mut_ptr() as *mut c_char);
    CStr::from_ptr(buf.as_ptr() as *const c_char)
      .to_string_lossy()
      .into_owned()
  }
}

/// Sets [DecQuad] to zero.
pub fn dec_quad_zero(dq: &mut DecQuad) {
  unsafe {
    decQuadZero(dq);
  }
}
