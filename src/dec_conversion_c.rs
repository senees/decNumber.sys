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

//! `C` bindings for decimal conversion functions.

use crate::{DecContext, DecDouble, DecNumber, DecQuad, DecSingle};

#[rustfmt::skip]
extern "C" {
  /// `C` binding to *decimal32ToNumber* function.
  pub fn decimal32ToNumber(d: *const DecSingle, n: *mut DecNumber) -> *mut DecNumber;
  /// `C` binding to *decimal32FromNumber* function.
  pub fn decimal32FromNumber(d: *mut DecSingle, n: *const DecNumber, dc: *mut DecContext) -> *mut DecSingle;
  /// `C` binding to *decimal64ToNumber* function.
  pub fn decimal64ToNumber(d: *const DecDouble, n: *mut DecNumber) -> *mut DecNumber;
  /// `C` binding to *decimal64FromNumber* function.
  pub fn decimal64FromNumber(d: *mut DecDouble, n: *const DecNumber, dc: *mut DecContext) -> *mut DecDouble;
  /// `C` binding to *decimal128ToNumber* function.
  pub fn decimal128ToNumber(d: *const DecQuad, n: *mut DecNumber) -> *mut DecNumber;
  /// `C` binding to *decimal128FromNumber* function.
  pub fn decimal128FromNumber(d: *mut DecQuad, n: *const DecNumber, dc: *mut DecContext) -> *mut DecQuad;
}
