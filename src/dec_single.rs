//!

use crate::dec_context::DecContext;
use crate::dec_conversion_c::{decimal32FromNumber, decimal32ToNumber};
use crate::dec_number::DecNumber;
use crate::dec_number_c::decNumberRescale;
use crate::dec_single_c::*;
use libc::c_char;
use std::ffi::{CStr, CString};

/// Length in bytes of [DecSingle] union.
const DEC_SINGLE_BYTES: usize = 4;

/// Maximum length of the [DecSingle] string.
const DEC_SINGLE_STRING: usize = 16;

/// Buffer for [DecSingle] string.
const DEC_SINGLE_STRING_BUFFER: [c_char; DEC_SINGLE_STRING] = [0; DEC_SINGLE_STRING];

/// Convenient constant for [DecQuad] equal to positive zero.
#[rustfmt::skip]
pub(crate) const DEC_SINGLE_POSITIVE_ZERO: DecSingle = {
  #[cfg(target_endian = "little")]
  { DecSingle { bytes: [0x00, 0x00, 0x50, 0x22] }}
  #[cfg(target_endian = "big")]
  { DecSingle { bytes: [0x22, 0x50, 0x00, 0x00] }}
};

/// The `decSingle` 32-bit type, accessible by all sizes.
#[repr(C)]
pub union DecSingle {
  pub bytes: [u8; DEC_SINGLE_BYTES],
  pub shorts: [u16; DEC_SINGLE_BYTES / 2],
  pub words: [u32; DEC_SINGLE_BYTES / 4],
}

impl Default for DecSingle {
  /// The default value for [DecSingle] is positive zero.
  fn default() -> Self {
    DEC_SINGLE_POSITIVE_ZERO
  }
}

// /// Adds two [DecSingles](DecSingle).
// pub fn dec_single_add(lhs: &DecSingle, rhs: &DecSingle, ctx: &mut DecContext) -> DecSingle {
//   let mut result = DecSingle::default();
//   let mut dl = DecDouble::default();
//   let mut dr = DecDouble::default();
//   let mut d_res = DecDouble::default();
//   unsafe {
//     decSingleToWider(lhs, &mut dl, ctx);
//     decSingleToWider(rhs, &mut dr, ctx);
//     decSingleAdd(&mut result, lhs, rhs, ctx);
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

/// Converts [DecSingle] from string.
pub fn dec_single_from_string(s: &str, ctx: &mut DecContext) -> DecSingle {
  let c_s = CString::new(s).unwrap();
  let mut result = DecSingle::default();
  unsafe {
    decSingleFromString(&mut result, c_s.as_ptr(), ctx);
  }
  result
}

///
pub fn dec_single_rescale(q1: &DecSingle, q2: &DecSingle, ctx: &mut DecContext) -> DecSingle {
  let mut res = DecSingle::default();
  let mut n1 = DecNumber::default();
  let mut n2 = DecNumber::default();
  let mut nr = DecNumber::default();
  unsafe {
    decimal32ToNumber(q1, &mut n1);
    decimal32ToNumber(q2, &mut n2);
    decNumberRescale(&mut nr, &n1, &n2, ctx);
    decimal32FromNumber(&mut res, &nr, ctx);
  }
  res
}

/// Converts [DecSingle] into [String].
pub fn dec_single_to_string(d: &DecSingle) -> String {
  unsafe {
    let mut buf = DEC_SINGLE_STRING_BUFFER;
    decSingleToString(d, buf.as_mut_ptr() as *mut c_char);
    CStr::from_ptr(buf.as_ptr() as *const c_char)
      .to_string_lossy()
      .into_owned()
  }
}

/// Sets [DecSingle] to the unsigned integer zero.
pub fn dec_single_zero(ds: &mut DecSingle) {
  unsafe {
    decSingleZero(ds);
  }
}
