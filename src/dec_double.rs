//!

use crate::dec_context::DecContext;
use crate::dec_double_c::*;

/// Length in bytes of [DecDouble] union.
const DEC_DOUBLE_BYTES: usize = 8;

/// Convenient constant for [DecQuad] equal to positive zero.
#[rustfmt::skip]
pub(crate) const DEC_DOUBLE_POSITIVE_ZERO: DecDouble = DecDouble {
  bytes: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x38, 0x22],
};

/// The `DecDouble` 32-bit type, accessible by all sizes.
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
    DEC_DOUBLE_POSITIVE_ZERO
  }
}

/// Adds two [DecDoubles](DecDouble).
pub fn dec_double_add(lhs: &DecDouble, rhs: &DecDouble, ctx: &mut DecContext) -> DecDouble {
  let mut res = DecDouble::default();
  unsafe {
    decDoubleAdd(&mut res, lhs, rhs, ctx);
  }
  res
}

/// Sets [DecDouble] to the unsigned integer zero.
pub fn dec_double_zero(res: &mut DecDouble) {
  unsafe {
    decDoubleZero(res);
  }
}
