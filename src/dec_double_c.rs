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

//! Unsafe bindings for 64-bit decimal.

use crate::{DecContext, DecDouble};
use libc::c_char;

#[rustfmt::skip]
extern "C" {
  /// Unsafe binding to *decDoubleAdd* function.
  pub fn decDoubleAdd(dd: *mut DecDouble, dd1: *const DecDouble, dd2: *const DecDouble, dc: *mut DecContext) -> *mut DecDouble;
  /// Unsafe binding to *decDoubleFromString* function.
  pub fn decDoubleFromString(dd: *mut DecDouble, s: *const c_char, dc: *mut DecContext) -> *mut DecDouble;
  /// Unsafe binding to *decDoubleToString* function.
  pub fn decDoubleToString(dd1: *const DecDouble, s: *mut c_char) -> *mut c_char;
  /// Unsafe binding to *decDoubleZero* function.
  pub fn decDoubleZero(dd: *mut DecDouble);
}
