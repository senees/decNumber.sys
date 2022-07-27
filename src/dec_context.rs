//!

use crate::dec_common::{Int, Ubyte, Uint};
use crate::dec_context_c::*;

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

/// Returns the default context for decimal arithmetic initialized
/// according the specified [ContextKind] value.
pub fn dec_context_default(kind: ContextKind) -> DecContext {
  let mut context = DecContext::default();
  unsafe {
    decContextDefault(&mut context, kind as Int);
  }
  context
}
