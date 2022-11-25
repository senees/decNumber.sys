/*
 * MIT License
 *
 * Copyright (c) 2022 senees
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

//! Safe bindings for decimal context.

use crate::dec_context_c::*;

/// Initializes context to ANSI X3-274 defaults.
pub const DEC_INIT_BASE: i32 = 0;

/// Initializes context to IEEE 754 defaults, 32-bit.
pub const DEC_INIT_DECIMAL32: i32 = 32;

/// Initializes context to IEEE 754 defaults, 64-bit.
pub const DEC_INIT_DECIMAL64: i32 = 64;

/// Initializes context to IEEE 754 defaults, 128-bit.
pub const DEC_INIT_DECIMAL128: i32 = 128;

/// Decimal context.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct DecContext {
  /// Working precision.
  pub digits: i32,
  /// Maximum positive exponent.
  pub emax: i32,
  /// Minimum negative exponent.
  pub emin: i32,
  /// Rounding mode.
  pub round: u32,
  /// Trap-enabler flags.
  pub traps: u32,
  /// Status flags.
  pub status: u32,
  /// Flag: apply IEEE exponent clamp.
  pub clamp: u8,
}

/// Returns decimal context initialized with maximum specified number of digits.
pub fn dec_context_base(digits: i32) -> DecContext {
  let mut context = DecContext::default();
  unsafe {
    decContextDefault(&mut context, DEC_INIT_BASE);
    context.digits = digits;
    context.traps = 0;
  }
  context
}

/// Returns decimal context initialized for 32-bit decimals.
pub fn dec_context_32() -> DecContext {
  let mut context = DecContext::default();
  unsafe {
    decContextDefault(&mut context, DEC_INIT_DECIMAL32);
    context.traps = 0;
  }
  context
}

/// Returns decimal context initialized for 64-bit decimals.
pub fn dec_context_64() -> DecContext {
  let mut context = DecContext::default();
  unsafe {
    decContextDefault(&mut context, DEC_INIT_DECIMAL64);
    context.traps = 0;
  }
  context
}

/// Returns decimal context initialized for 128-bit decimals.
pub fn dec_context_128() -> DecContext {
  let mut context = DecContext::default();
  unsafe {
    decContextDefault(&mut context, DEC_INIT_DECIMAL128);
    context.traps = 0;
  }
  context
}

/// Safe binding to *decContextDefault* function.
pub fn dec_context_default(kind: i32) -> DecContext {
  unsafe {
    let mut context = DecContext::default();
    decContextDefault(&mut context, kind);
    context.traps = 0;
    context
  }
}

/// Safe binding to *decContextZeroStatus* function.
pub fn dec_context_zero_status(dc: &mut DecContext) {
  unsafe {
    decContextZeroStatus(dc);
  }
}
