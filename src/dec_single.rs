//!

use crate::dec_context::DecContext;
use crate::dec_single_c::*;

/// Length in bytes of [DecSingle] union.
const DEC_SINGLE_BYTES: usize = 4;

/// Convenient constant for [DecQuad] equal to positive zero.
#[rustfmt::skip]
pub(crate) const DEC_SINGLE_POSITIVE_ZERO: DecSingle = DecSingle {
  bytes: [0x00, 0x00, 0x50, 0x22],
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

/// Adds two [DecSingles](DecSingle).
pub fn dec_single_add(dql: &DecSingle, dqr: &DecSingle, ctx: &mut DecContext) -> DecSingle {
  let mut result = DecSingle::default();
  unsafe {
    decSingleAdd(&mut result, dql, dqr, ctx);
  }
  result
}

/// Sets [DecSingle] to the unsigned integer zero.
pub fn dec_single_zero(ds: &mut DecSingle) {
  unsafe {
    decSingleZero(ds);
  }
}
