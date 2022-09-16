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

//! Unsafe bindings for arbitrary precision decimal.

use crate::{DecContext, DecNumber};
use libc::{c_char, c_int, c_uchar, c_uint};

#[rustfmt::skip]
extern "C" {
  /// Unsafe binding to *decNumberAdd* function.
  pub fn decNumberAdd(dn: *mut DecNumber, dn1: *const DecNumber, dn2: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberCompare* function.
  pub fn decNumberCompare(dn: *mut DecNumber, dn1: *const DecNumber, dn2: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberDivide* function.
  pub fn decNumberDivide(dn: *mut DecNumber, dn1: *const DecNumber, dn2: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberExp* function.
  pub fn decNumberExp(dn: *mut DecNumber, dn1: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberFromInt32* function.
  pub fn decNumberFromInt32(dn: *mut DecNumber, n: c_int) -> *mut DecNumber;
  /// Unsafe binding to *decNumberFromString* function.
  pub fn decNumberFromString(dn: *mut DecNumber, s: *const c_char, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberFromUInt32* function.
  pub fn decNumberFromUInt32(dn: *mut DecNumber, n: c_uint) -> *mut DecNumber;
  /// Unsafe binding to *decNumberLn* function.
  pub fn decNumberLn(dn: *mut DecNumber, dn1: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberMinus* function.
  pub fn decNumberMinus(dn: *mut DecNumber, dn1: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberMultiply* function.
  pub fn decNumberMultiply(dn: *mut DecNumber, dn1: *const DecNumber, dn2: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberPlus* function.
  pub fn decNumberPlus(dn: *mut DecNumber, dn1: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberPower* function.
  pub fn decNumberPower(dn: *mut DecNumber, dn1: *const DecNumber, dn2: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberQuantize* function.
  pub fn decNumberQuantize(dn: *mut DecNumber, dn1: *const DecNumber, dn2: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberReduce* function.
  pub fn decNumberReduce(dn: *mut DecNumber, dn1: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberRescale* function.
  pub fn decNumberRescale(dn: *mut DecNumber, dn1: *const DecNumber, dn2: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberScaleB* function.
  pub fn decNumberScaleB(dn: *mut DecNumber, dn1: *const DecNumber, dn2: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberSetBCD* function.
  pub fn decNumberSetBCD(dn: *mut DecNumber, bcd: *const c_uchar, n: c_uint) -> *mut DecNumber;
  /// Unsafe binding to *decNumberSquareRoot* function.
  pub fn decNumberSquareRoot(dn: *mut DecNumber, dn1: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberSubtract* function.
  pub fn decNumberSubtract(dn: *mut DecNumber, dn1: *const DecNumber, dn2: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberToString* function.
  pub fn decNumberToString(dn1: *const DecNumber, s: *mut c_char) -> *mut c_char;
  /// Unsafe binding to *decNumberZero* function.
  pub fn decNumberZero(dn: *mut DecNumber);
}
