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

/*
  pub fn decDoubleAbs() {}
  pub fn decDoubleAnd() {}
  pub fn decDoubleCanonical() {}
  pub fn decDoubleClass() {}
  pub fn decDoubleClassString() {}
  pub fn decDoubleCompare() {}
  pub fn decDoubleCompareSignal() {}
  pub fn decDoubleCompareTotal() {}
  pub fn decDoubleCompareTotalMag() {}
  pub fn decDoubleCopy() {}
  pub fn decDoubleCopyAbs() {}
  pub fn decDoubleCopyNegate() {}
  pub fn decDoubleCopySign() {}
  pub fn decDoubleDigits() {}
  pub fn decDoubleDivide() {}
  pub fn decDoubleDivideInteger() {}
  pub fn decDoubleFMA() {}
  pub fn decDoubleFromBCD() {}
  pub fn decDoubleFromInt32() {}
  pub fn decDoubleFromNumber() {}
  pub fn decDoubleFromPacked() {}
  pub fn decDoubleFromPackedChecked() {}
  pub fn decDoubleFromUInt32() {}
  pub fn decDoubleFromWider() {}
  pub fn decDoubleGetCoefficient() {}
  pub fn decDoubleGetExponent() {}
  pub fn decDoubleInvert() {}
  pub fn decDoubleIsCanonical() {}
  pub fn decDoubleIsFinite() {}
  pub fn decDoubleIsInfinite() {}
  pub fn decDoubleIsInteger() {}
  pub fn decDoubleIsLogical() {}
  pub fn decDoubleIsNaN() {}
  pub fn decDoubleIsNegative() {}
  pub fn decDoubleIsNormal() {}
  pub fn decDoubleIsPositive() {}
  pub fn decDoubleIsSignaling() {}
  pub fn decDoubleIsSignalling() {}
  pub fn decDoubleIsSigned() {}
  pub fn decDoubleIsSubnormal() {}
  pub fn decDoubleIsZero() {}
  pub fn decDoubleLogB() {}
  pub fn decDoubleMax() {}
  pub fn decDoubleMaxMag() {}
  pub fn decDoubleMin() {}
  pub fn decDoubleMinMag() {}
  pub fn decDoubleMinus() {}
  pub fn decDoubleMultiply() {}
  pub fn decDoubleNextMinus() {}
  pub fn decDoubleNextPlus() {}
  pub fn decDoubleNextToward() {}
  pub fn decDoubleOr() {}
  pub fn decDoublePlus() {}
  pub fn decDoubleQuantize() {}
  pub fn decDoubleRadix() {}
  pub fn decDoubleReduce() {}
  pub fn decDoubleRemainder() {}
  pub fn decDoubleRemainderNear() {}
  pub fn decDoubleRotate() {}
  pub fn decDoubleSameQuantum() {}
  pub fn decDoubleScaleB() {}
  pub fn decDoubleSetCoefficient() {}
  pub fn decDoubleSetExponent() {}
  pub fn decDoubleShift() {}
  pub fn decDoubleShow() {}
  pub fn decDoubleSubtract() {}
  pub fn decDoubleToBCD() {}
  pub fn decDoubleToEngString() {}
  pub fn decDoubleToInt32() {}
  pub fn decDoubleToInt32Exact() {}
  pub fn decDoubleToIntegralExact() {}
  pub fn decDoubleToIntegralValue() {}
  pub fn decDoubleToNumber() {}
  pub fn decDoubleToPacked() {}
  pub fn decDoubleToUInt32() {}
  pub fn decDoubleToUInt32Exact() {}
  pub fn decDoubleToWider() {}
  pub fn decDoubleVersion() {}
  pub fn decDoubleXor() {}
  pub fn decDoubleToUInt32() {}
  pub fn decDoubleToUInt32Exact() {}
  pub fn decDoubleToWider() {}
  pub fn decDoubleVersion() {}
  pub fn decDoubleXor() {}
*/
