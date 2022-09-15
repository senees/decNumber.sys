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
  pub fn decNumberAdd(res: *mut DecNumber, dn1: *const DecNumber, dn2: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberCompare* function.
  pub fn decNumberCompare(res: *mut DecNumber, dn1: *const DecNumber, dn2: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberDivide* function.
  pub fn decNumberDivide(res: *mut DecNumber, dn1: *const DecNumber, dn2: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberExp* function.
  pub fn decNumberExp(res: *mut DecNumber, dn: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberFromInt32* function.
  pub fn decNumberFromInt32(res: *mut DecNumber, n: c_int) -> *mut DecNumber;
  /// Unsafe binding to *decNumberFromString* function.
  pub fn decNumberFromString(res: *mut DecNumber, s: *const c_char, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberFromUInt32* function.
  pub fn decNumberFromUInt32(res: *mut DecNumber, n: c_uint) -> *mut DecNumber;
  /// Unsafe binding to *decNumberLn* function.
  pub fn decNumberLn(res: *mut DecNumber, dn: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberMinus* function.
  pub fn decNumberMinus(res: *mut DecNumber, dn: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberMultiply* function.
  pub fn decNumberMultiply(res: *mut DecNumber, dn1: *const DecNumber, dn2: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberPlus* function.
  pub fn decNumberPlus(res: *mut DecNumber, dn: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberPower* function.
  pub fn decNumberPower(res: *mut DecNumber, dn1: *const DecNumber, dn2: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberQuantize* function.
  pub fn decNumberQuantize(res: *mut DecNumber, dn1: *const DecNumber, dn2: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberReduce* function.
  pub fn decNumberReduce(res: *mut DecNumber, dn: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberRescale* function.
  pub fn decNumberRescale(res: *mut DecNumber, dn1: *const DecNumber, dn2: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberScaleB* function.
  pub fn decNumberScaleB(res: *mut DecNumber, dn1: *const DecNumber, dn2: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberSetBCD* function.
  pub fn decNumberSetBCD(res: *mut DecNumber, bcd: *const c_uchar, n: c_uint) -> *mut DecNumber;
  /// Unsafe binding to *decNumberSquareRoot* function.
  pub fn decNumberSquareRoot(res: *mut DecNumber, dn: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberSubtract* function.
  pub fn decNumberSubtract(res: *mut DecNumber, dn1: *const DecNumber, dn2: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberToString* function.
  pub fn decNumberToString(dn: *const DecNumber, s: *mut c_char) -> *mut c_char;
  /// Unsafe binding to *decNumberZero* function.
  pub fn decNumberZero(res: *mut DecNumber);
}
