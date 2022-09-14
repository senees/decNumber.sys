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

//! Decimal context definitions.

use crate::dec_context_c::*;

/// Initializes context to ANSI X3-274 defaults.
pub const DEC_INIT_BASE: i32 = 0;

/// Initializes context to IEEE 754 defaults, 32-bit.
pub const DEC_INIT_DECIMAL32: i32 = 32;

/// Initializes context to IEEE 754 defaults, 64-bit.
pub const DEC_INIT_DECIMAL64: i32 = 64;

/// Initializes context to IEEE 754 defaults, 128-bit.
pub const DEC_INIT_DECIMAL128: i32 = 128;

/// Enumeration of context initialization constants.
#[repr(i32)]
pub enum ContextKind {
  /// ANSI X3-274 defaults with maximum number of digits.
  Base(i32),
  /// IEEE 754 defaults, 32-bit.
  Decimal32,
  /// IEEE 754 defaults, 64-bit.
  Decimal64,
  /// IEEE 754 defaults, 128-bit.
  Decimal128,
}

/// Decimal context.
#[repr(C)]
#[derive(Default, Clone)]
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

/// Returns decimal context initialized according the specified [ContextKind] value.
pub fn dec_context_default(kind: ContextKind) -> DecContext {
  let mut context = DecContext::default();
  unsafe {
    match kind {
      ContextKind::Base(digits) => {
        decContextDefault(&mut context, DEC_INIT_BASE);
        context.digits = digits;
        context.traps = 0;
      }
      ContextKind::Decimal32 => {
        decContextDefault(&mut context, DEC_INIT_DECIMAL32);
        context.traps = 0;
      }
      ContextKind::Decimal64 => {
        decContextDefault(&mut context, DEC_INIT_DECIMAL64);
        context.traps = 0;
      }
      ContextKind::Decimal128 => {
        decContextDefault(&mut context, DEC_INIT_DECIMAL128);
        context.traps = 0;
      }
    }
  }
  context
}
