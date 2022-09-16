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

//! Unsafe bindings for 128-bit decimal.

use crate::{DecContext, DecQuad};
use libc::{c_char, c_int, c_uchar, c_uint};

#[rustfmt::skip]
extern "C" {
  /// Unsafe binding to *decQuadAbs* function.
  pub fn decQuadAbs(dq: *mut DecQuad, dq1: *const DecQuad, dc: *mut DecContext) -> *mut DecQuad;
  /// Unsafe binding to *decQuadAdd* function.
  pub fn decQuadAdd(dq: *mut DecQuad, dq1: *const DecQuad, dq2: *const DecQuad, dc: *mut DecContext) -> *mut DecQuad;
  /// Unsafe binding to *decQuadAnd* function.
  pub fn decQuadAnd(dq: *mut DecQuad, dq1: *const DecQuad, dq2: *const DecQuad, dc: *mut DecContext) -> *mut DecQuad;
  /// Unsafe binding to *decQuadCompare* function.
  pub fn decQuadCompare(dq: *mut DecQuad, dq1: *const DecQuad, dq2: *const DecQuad, dc: *mut DecContext) -> *mut DecQuad;
  /// Unsafe binding to *decQuadDivide* function.
  pub fn decQuadDivide(dq: *mut DecQuad, dq1: *const DecQuad, dq2: *const DecQuad, dc: *mut DecContext) -> *mut DecQuad;
  /// Unsafe binding to *decQuadFromBCD* function.
  pub fn decQuadFromBCD(dq: *mut DecQuad, exp: c_int, bcd: *const c_uchar, sign: c_int) -> *mut DecQuad;
  /// Unsafe binding to *decQuadFromInt32* function.
  pub fn decQuadFromInt32(dq: *mut DecQuad, n: c_int) -> *mut DecQuad;
  /// Unsafe binding to *decQuadFromString* function.
  pub fn decQuadFromString(dq: *mut DecQuad, s: *const c_char, dc: *mut DecContext) -> *mut DecQuad;
  /// Unsafe binding to *decQuadFromUInt32* function.
  pub fn decQuadFromUInt32(dq: *mut DecQuad, n: c_uint) -> *mut DecQuad;
  /// Unsafe binding to *decQuadIsFinite* function.
  pub fn decQuadIsFinite(arg1: *const DecQuad) -> c_uint;
  /// Unsafe binding to *decQuadIsInteger* function.
  pub fn decQuadIsInteger(arg1: *const DecQuad) -> c_uint;
  /// Unsafe binding to *decQuadIsNegative* function.
  pub fn decQuadIsNegative(arg1: *const DecQuad) -> c_uint;
  /// Unsafe binding to *decQuadIsPositive* function.
  pub fn decQuadIsPositive(arg1: *const DecQuad) -> c_uint;
  /// Unsafe binding to *decQuadIsZero* function.
  pub fn decQuadIsZero(arg1: *const DecQuad) -> c_uint;
  /// Unsafe binding to *decQuadMinus* function.
  pub fn decQuadMinus(dq: *mut DecQuad, dq1: *const DecQuad, dc: *mut DecContext) -> *mut DecQuad;
  /// Unsafe binding to *decQuadMultiply* function.
  pub fn decQuadMultiply(dq: *mut DecQuad, dq1: *const DecQuad, dq2: *const DecQuad, dc: *mut DecContext) -> *mut DecQuad;
  /// Unsafe binding to *decQuadPlus* function.
  pub fn decQuadPlus(dq: *mut DecQuad, dq1: *const DecQuad, dc: *mut DecContext) -> *mut DecQuad;
  /// Unsafe binding to *decQuadQuantize* function.
  pub fn decQuadQuantize(dq: *mut DecQuad, dq1: *const DecQuad, dq2: *const DecQuad, dc: *mut DecContext) -> *mut DecQuad;
  /// Unsafe binding to *decQuadReduce* function.
  pub fn decQuadReduce(dq: *mut DecQuad, dq1: *const DecQuad, dc: *mut DecContext) -> *mut DecQuad;
  /// Unsafe binding to *decQuadRemainder* function.
  pub fn decQuadRemainder(dq: *mut DecQuad, dq1: *const DecQuad, dq2: *const DecQuad, dc: *mut DecContext) -> *mut DecQuad;
  /// Unsafe binding to *decQuadScaleB* function.
  pub fn decQuadScaleB(dq: *mut DecQuad, dq1: *const DecQuad, dq2: *const DecQuad, dc: *mut DecContext) -> *mut DecQuad;
  /// Unsafe binding to *decQuadSubtract* function.
  pub fn decQuadSubtract(dq: *mut DecQuad, dq1: *const DecQuad, dq2: *const DecQuad, dc: *mut DecContext) -> *mut DecQuad;
  /// Unsafe binding to *decQuadToIntegralValue* function.
  pub fn decQuadToIntegralValue(dq: *mut DecQuad, dq1: *const DecQuad, dc: *mut DecContext, rounding: u32) -> *mut DecQuad;
  /// Unsafe binding to *decQuadToString* function.
  pub fn decQuadToString(dq: *const DecQuad, s: *mut c_char) -> *mut c_char;
  /// Unsafe binding to *decQuadZero* function.
  pub fn decQuadZero(dq: *mut DecQuad);
}

/*

  pub fn decQuadCanonical(){}
  pub fn decQuadClass(){}
  pub fn decQuadClassString(){}
  pub fn decQuadCompareSignal(){}
  pub fn decQuadCompareTotal(){}
  pub fn decQuadCompareTotalMag(){}
  pub fn decQuadCopy(){}
  pub fn decQuadCopyAbs(){}
  pub fn decQuadCopyNegate(){}
  pub fn decQuadCopySign(){}
  pub fn decQuadDigits(){}
  pub fn decQuadDivideInteger(){}
  pub fn decQuadFMA(){}
  pub fn decQuadFromNumber(){}
  pub fn decQuadFromPacked(){}
  pub fn decQuadFromPackedChecked(){}
  pub fn decQuadFromWider(){}
  pub fn decQuadGetCoefficient(){}
  pub fn decQuadGetExponent(){}
  pub fn decQuadInvert(){}
  pub fn decQuadIsCanonical(){}
  pub fn decQuadIsInfinite(){}
  pub fn decQuadIsLogical(){}
  pub fn decQuadIsNaN(){}
  pub fn decQuadIsNormal(){}
  pub fn decQuadIsSignaling(){}
  pub fn decQuadIsSignalling(){}
  pub fn decQuadIsSigned(){}
  pub fn decQuadIsSubnormal(){}
  pub fn decQuadLogB(){}
  pub fn decQuadMax(){}
  pub fn decQuadMaxMag(){}
  pub fn decQuadMin(){}
  pub fn decQuadMinMag(){}
  pub fn decQuadNextMinus(){}
  pub fn decQuadNextPlus(){}
  pub fn decQuadNextToward(){}
  pub fn decQuadOr(){}
  pub fn decQuadRadix(){}
  pub fn decQuadRemainderNear(){}
  pub fn decQuadRotate(){}
  pub fn decQuadSameQuantum(){}
  pub fn decQuadSetCoefficient(){}
  pub fn decQuadSetExponent(){}
  pub fn decQuadShift(){}
  pub fn decQuadShow(){}
  pub fn decQuadToBCD(){}
  pub fn decQuadToEngString(){}
  pub fn decQuadToInt32(){}
  pub fn decQuadToInt32Exact(){}
  pub fn decQuadToIntegralExact(){}
  pub fn decQuadToNumber(){}
  pub fn decQuadToPacked(){}
  pub fn decQuadToUInt32(){}
  pub fn decQuadToUInt32Exact(){}
  pub fn decQuadToWider(){}
  pub fn decQuadVersion(){}
  pub fn decQuadXor(){}
  pub fn decQuadToUInt32(){}
  pub fn decQuadToUInt32Exact(){}
  pub fn decQuadToWider(){}
  pub fn decQuadVersion(){}
  pub fn decQuadXor(){}
  pub fn decQuadZero(){}
*/
