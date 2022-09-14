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

//! Decimal conversion functions.

use crate::dec_conversion_c::*;
use crate::dec_double::DecDouble;
use crate::dec_number::DecNumber;
use crate::dec_quad::DecQuad;
use crate::dec_single::DecSingle;
use crate::DecContext;

///
pub fn decimal32_to_number(d: &DecSingle, n: &mut DecNumber) {
  unsafe {
    decimal32ToNumber(d, n);
  }
}

///
pub fn decimal32_from_number(n: &DecNumber, ctx: &mut DecContext) -> DecSingle {
  let mut res = DecSingle::default();
  unsafe {
    decimal32FromNumber(&mut res, n, ctx);
  }
  res
}

///
pub fn decimal64_to_number(d: &DecDouble, n: &mut DecNumber) {
  unsafe {
    decimal64ToNumber(d, n);
  }
}

///
pub fn decimal64_from_number(n: &DecNumber, ctx: &mut DecContext) -> DecDouble {
  let mut res = DecDouble::default();
  unsafe {
    decimal64FromNumber(&mut res, n, ctx);
  }
  res
}

///
pub fn decimal128_to_number(d: &DecQuad, n: &mut DecNumber) {
  unsafe {
    decimal128ToNumber(d, n);
  }
}

///
pub fn decimal128_from_number(n: &DecNumber, ctx: &mut DecContext) -> DecQuad {
  let mut res = DecQuad::default();
  unsafe {
    decimal128FromNumber(&mut res, n, ctx);
  }
  res
}
