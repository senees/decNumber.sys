//! Rust bindings to functions and structures from `The decNumber C Library`.  

use crate::dec_number::*;

/// Convenient alias for [i32] numbers.
pub type Int = i32;

/// Convenient alias for [u32] numbers.
pub type Uint = u32;

/// Convenient alias for [u8] numbers.
pub type Ubyte = u8;

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
  digits: Int,
  emax: Int,
  emin: Int,
  round: Uint,
  traps: Uint,
  status: Uint,
  clamp: Ubyte,
}

impl DecContext {
  pub fn digits(&self) -> &Int {
    &self.digits
  }
  pub fn emax(&self) -> &Int {
    &self.emax
  }
  pub fn emin(&self) -> &Int {
    &self.emin
  }
  pub fn round(&self) -> &Uint {
    &self.round
  }
  pub fn traps(&self) -> &Uint {
    &self.traps
  }
  pub fn status(&self) -> &Uint {
    &self.status
  }
  pub fn clamp(&self) -> &Ubyte {
    &self.clamp
  }
}

/// Returns the default context for decimal arithmetic.
pub fn dec_context_default(kind: ContextKind) -> DecContext {
  let mut context = DecContext::default();
  unsafe {
    decContextDefault(&mut context, kind as Int);
  }
  context
}
