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

/// Safe binding to *decimal32ToNumber* function.
pub fn decimal32_to_number(ds: &DecSingle, dn: &mut DecNumber) {
  unsafe {
    decimal32ToNumber(ds, dn);
  }
}

/// Safe binding to *decimal32FromNumber* function.
pub fn decimal32_from_number(dn: &DecNumber, dc: &mut DecContext) -> DecSingle {
  let mut ds_res = DecSingle::default();
  unsafe {
    decimal32FromNumber(&mut ds_res, dn, dc);
  }
  ds_res
}

/// Safe binding to *decimal64ToNumber* function.
pub fn decimal64_to_number(dd: &DecDouble, dn: &mut DecNumber) {
  unsafe {
    decimal64ToNumber(dd, dn);
  }
}

/// Safe binding to *decimal64FromNumber* function.
pub fn decimal64_from_number(dn: &DecNumber, dc: &mut DecContext) -> DecDouble {
  let mut dd_res = DecDouble::default();
  unsafe {
    decimal64FromNumber(&mut dd_res, dn, dc);
  }
  dd_res
}

/// Safe binding to *decimal128ToNumber* function.
pub fn decimal128_to_number(dq: &DecQuad, dn: &mut DecNumber) {
  unsafe {
    decimal128ToNumber(dq, dn);
  }
}

/// Safe binding to *decimal128FromNumber* function.
pub fn decimal128_from_number(dn: &DecNumber, dc: &mut DecContext) -> DecQuad {
  let mut dq_res = DecQuad::default();
  unsafe {
    decimal128FromNumber(&mut dq_res, dn, dc);
  }
  dq_res
}
