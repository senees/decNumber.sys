//! Rust bindings to functions and structures from `The decNumber C Library`.  

use crate::dec_number_c::*;
use libc::c_char;
use std::ffi::{CStr, CString};

//--------------------------------------------------------------------------------------------------
// Type aliases
//--------------------------------------------------------------------------------------------------

/// Convenient alias for [i32] numbers.
pub type Int = i32;

/// Convenient alias for [u32] numbers.
pub type Uint = u32;

/// Convenient alias for [u8] numbers.
pub type Ubyte = u8;

//--------------------------------------------------------------------------------------------------
// Context initialization constants
//--------------------------------------------------------------------------------------------------

/// Initializes context to ANSI X3-274 defaults.
const DEC_INIT_BASE: Int = 0;

/// Initializes context to IEEE 754 defaults, 32-bit.
const DEC_INIT_DECIMAL32: Int = 32;

/// Initializes context to IEEE 754 defaults, 64-bit.
const DEC_INIT_DECIMAL64: Int = 64;

/// Initializes context to IEEE 754 defaults, 128-bit.
const DEC_INIT_DECIMAL128: Int = 128;

/// Convenient enumeration of context initialization constants.
#[repr(i32)]
pub enum ContextKind {
  Base = DEC_INIT_BASE,
  Decimal32 = DEC_INIT_DECIMAL32,
  Decimal64 = DEC_INIT_DECIMAL64,
  Decimal128 = DEC_INIT_DECIMAL128,
}

//--------------------------------------------------------------------------------------------------
// Context
//--------------------------------------------------------------------------------------------------

/// Decimal context.
#[repr(C)]
#[derive(Default, Clone)]
pub struct DecContext {
  /// Working precision.
  digits: Int,
  /// Maximum positive exponent.
  emax: Int,
  /// Minimum negative exponent.
  emin: Int,
  /// Rounding mode.
  round: Uint,
  /// Trap-enabler flags.
  traps: Uint,
  /// Status flags.
  status: Uint,
  /// flag: apply IEEE exponent clamp.
  clamp: Ubyte,
}

impl DecContext {
  /// Returns reference to digits.
  pub fn digits(&self) -> &Int {
    &self.digits
  }
  /// Returns reference to emax.
  pub fn emax(&self) -> &Int {
    &self.emax
  }
  /// Returns reference to emin.
  pub fn emin(&self) -> &Int {
    &self.emin
  }
  /// Returns reference to rounding flag.
  pub fn round(&self) -> &Uint {
    &self.round
  }
  /// Returns reference to traps flag.
  pub fn traps(&self) -> &Uint {
    &self.traps
  }
  /// Returns reference to status.
  pub fn status(&self) -> &Uint {
    &self.status
  }
  /// Returns reference to clamp flag.
  pub fn clamp(&self) -> &Ubyte {
    &self.clamp
  }
}

//--------------------------------------------------------------------------------------------------
// DecNumber
//--------------------------------------------------------------------------------------------------

#[repr(C)]
#[derive(Default)]
pub struct DecNumber {
  digits: i32,
  exponent: i32,
  bits: u8,
  lsu: [u16; 12],
}

//--------------------------------------------------------------------------------------------------
// DecQuad
//--------------------------------------------------------------------------------------------------

/// Length in bytes of DecQuad union.
const DEC_QUAD_BYTES: usize = 16;

/// Maximum length of the [DecQuad] string.
const DEC_QUAD_STRING: usize = 43;

/// Buffer for [DecQuad] string.
const DEC_QUAD_STRING_BUFFER: [c_char; DEC_QUAD_STRING] = [0; DEC_QUAD_STRING];

/// Positive zero for DecQuad.
#[rustfmt::skip]
pub(crate) const DEC_QUAD_POSITIVE_ZERO: DecQuad = DecQuad {
  bytes: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x22],
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

/// Returns the default context for decimal arithmetic initialized
/// according the specified [ContextKind] value.
pub fn dec_context_default(kind: ContextKind) -> DecContext {
  let mut context = DecContext::default();
  unsafe {
    decContextDefault(&mut context, kind as Int);
  }
  context
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

/// Adds two [DecQuads](DecQuad).
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
  let mut qr = DecQuad::default();
  let mut n1 = DecNumber::default();
  let mut n2 = DecNumber::default();
  let mut nr = DecNumber::default();
  unsafe {
    decimal128ToNumber(q1, &mut n1);
    decimal128ToNumber(q2, &mut n2);
    decNumberRescale(&mut nr, &n1, &n2, ctx);
    decimal128FromNumber(&mut qr, &nr, ctx);
  }
  qr
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
