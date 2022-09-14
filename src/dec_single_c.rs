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

//! `C` bindings for 32-bit decimal.

use crate::{DecContext, DecDouble, DecSingle};
use libc::c_char;

#[rustfmt::skip]
extern "C" {
  /// Unsafe binding to *decSingleFromString* function.
  pub fn decSingleFromString(res: *mut DecSingle, s: *const c_char, dc: *mut DecContext) -> *mut DecSingle;
  /// Unsafe binding to *decSingleFromWider* function.
  pub fn decSingleFromWider(res: *mut DecSingle, ds: *const DecDouble, dc: *mut DecContext) -> *mut DecSingle;
  /// Unsafe binding to *decSingleToString* function.
  pub fn decSingleToString(ds: *const DecSingle, s: *mut c_char) -> *mut c_char;
  /// Unsafe binding to *decSingleToWider* function.
  pub fn decSingleToWider(res: *const DecSingle, ds: *mut DecDouble, dc: *mut DecContext) -> *mut DecDouble;
  /// Unsafe binding to *decSingleZero* function.
  pub fn decSingleZero(res: *mut DecSingle);
}
