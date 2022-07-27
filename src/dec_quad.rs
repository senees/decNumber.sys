//!

use crate::dec_context::DecContext;
use crate::dec_conversion_c::*;
use crate::dec_number::DecNumber;
use crate::dec_number_c::*;
use crate::dec_quad_c::*;
use libc::c_char;
use std::ffi::{CStr, CString};
use std::fmt::Debug;

/// Length in bytes of [DecQuad] union.
const DEC_QUAD_BYTES: usize = 16;

/// Maximum length of the [DecQuad] string.
const DEC_QUAD_STRING: usize = 43;

/// Buffer for [DecQuad] string.
const DEC_QUAD_STRING_BUFFER: [c_char; DEC_QUAD_STRING] = [0; DEC_QUAD_STRING];

/// Convenient constant for [DecQuad] equal to positive zero.
#[rustfmt::skip]
pub(crate) const DEC_QUAD_POSITIVE_ZERO: DecQuad = {
  #[cfg(target_endian = "little")]
  { DecQuad { bytes: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x22] }}
  #[cfg(target_endian = "big")]
  { DecQuad { bytes: [0x22, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00] }}
};

/// The `decQuad` decimal 128-bit type, accessible by all sizes.
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
    DEC_QUAD_POSITIVE_ZERO
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

/// Adds two [DecQuads](DecQuad).
pub fn dec_quad_add(dql: &DecQuad, dqr: &DecQuad, ctx: &mut DecContext) -> DecQuad {
  let mut result = DecQuad::default();
  unsafe {
    decQuadAdd(&mut result, dql, dqr, ctx);
  }
  result
}

///
pub fn dec_quad_from_i32(n: i32) -> DecQuad {
  let mut result = DecQuad::default();
  unsafe {
    decQuadFromInt32(&mut result, n);
  }
  result
}

///
pub fn dec_quad_from_u32(n: u32) -> DecQuad {
  let mut result = DecQuad::default();
  unsafe {
    decQuadFromUInt32(&mut result, n);
  }
  result
}

/// Converts [DecQuad] from string.
pub fn dec_quad_from_string(s: &str, ctx: &mut DecContext) -> DecQuad {
  let c_s = CString::new(s).unwrap();
  let mut result = DecQuad::default();
  unsafe {
    decQuadFromString(&mut result, c_s.as_ptr(), ctx);
  }
  result
}

///
pub fn dec_quad_rescale(q1: &DecQuad, q2: &DecQuad, ctx: &mut DecContext) -> DecQuad {
  let mut res = DecQuad::default();
  let mut n1 = DecNumber::default();
  let mut n2 = DecNumber::default();
  let mut nr = DecNumber::default();
  unsafe {
    decimal128ToNumber(q1, &mut n1);
    decimal128ToNumber(q2, &mut n2);
    decNumberRescale(&mut nr, &n1, &n2, ctx);
    decimal128FromNumber(&mut res, &nr, ctx);
  }
  res
}

/// Converts [DecQuad] into [String].
pub fn dec_quad_to_string(d: &DecQuad) -> String {
  unsafe {
    let mut buf = DEC_QUAD_STRING_BUFFER;
    decQuadToString(d, buf.as_mut_ptr() as *mut c_char);
    CStr::from_ptr(buf.as_ptr() as *const c_char)
      .to_string_lossy()
      .into_owned()
  }
}

/// Sets [DecQuad] to the unsigned integer zero.
pub fn dec_quad_zero(ds: &mut DecQuad) {
  unsafe {
    decQuadZero(ds);
  }
}
