use crate::dec_number::*;

pub const DEC_INIT_DECQUAD: i32 = 128;

#[repr(C)]
#[derive(Default, Clone)]
pub struct DecContext {
  digits: i32,
  emax: i32,
  emin: i32,
  round: u32,
  traps: u32,
  status: u32,
  clamp: u8,
}

/// Returns the default context for decimal arithmetic.
pub fn dec_context_default() -> DecContext {
  let mut context = DecContext::default();
  unsafe {
    decContextDefault(&mut context, DEC_INIT_DECQUAD);
  }
  context
}
