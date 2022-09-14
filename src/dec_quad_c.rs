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

//! `C` bindings for 128-bit decimal.

use crate::{DecContext, DecQuad};
use libc::{c_char, c_int, c_uint};

#[rustfmt::skip]
extern "C" {
  /// `C` binding to *decQuadAdd* function.
  pub fn decQuadAdd(res: *mut DecQuad, lhs: *const DecQuad, rhs: *const DecQuad, ctx: *mut DecContext) -> *mut DecQuad;
  /// `C` binding to *decQuadFromInt32* function.
  pub fn decQuadFromInt32(dq: *mut DecQuad, n: c_int) -> *mut DecQuad;
  /// `C` binding to *decQuadFromUInt32* function.
  pub fn decQuadFromUInt32(dq: *mut DecQuad, n: c_uint) -> *mut DecQuad;
  /// `C` binding to *decQuadFromString* function.
  pub fn decQuadFromString(res: *mut DecQuad, s: *const c_char, ctx: *mut DecContext) -> *mut DecQuad;
  /// `C` binding to *decQuadToString* function.
  pub fn decQuadToString(dq: *const DecQuad, s: *mut c_char) -> *mut c_char;
  /// `C` binding to *decQuadZero* function.
  pub fn decQuadZero(res: *mut DecQuad);
}
