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

//! Unsafe bindings for decimal conversion functions.

use crate::{DecContext, DecDouble, DecNumber, DecQuad, DecSingle};

#[rustfmt::skip]
extern "C" {
  /// Unsafe binding to *decimal32ToNumber* function.
  pub fn decimal32ToNumber(ds: *const DecSingle, dn: *mut DecNumber) -> *mut DecNumber;
  /// Unsafe binding to *decimal32FromNumber* function.
  pub fn decimal32FromNumber(ds: *mut DecSingle, dn: *const DecNumber, dc: *mut DecContext) -> *mut DecSingle;
  /// Unsafe binding to *decimal64ToNumber* function.
  pub fn decimal64ToNumber(ds: *const DecDouble, dn: *mut DecNumber) -> *mut DecNumber;
  /// Unsafe binding to *decimal64FromNumber* function.
  pub fn decimal64FromNumber(ds: *mut DecDouble, dn: *const DecNumber, dc: *mut DecContext) -> *mut DecDouble;
  /// Unsafe binding to *decimal128ToNumber* function.
  pub fn decimal128ToNumber(dq: *const DecQuad, dn: *mut DecNumber) -> *mut DecNumber;
  /// Unsafe binding to *decimal128FromNumber* function.
  pub fn decimal128FromNumber(dq: *mut DecQuad, dn: *const DecNumber, dc: *mut DecContext) -> *mut DecQuad;
}
