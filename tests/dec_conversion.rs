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

use dec_number_sys::*;

#[test]
fn test_decimal32_to_number() {
  let ds = dec_single_from_string("34", &mut dec_context_32());
  let mut dn = DecNumber::default();
  let n = decimal32_to_number(&ds, &mut dn);
  assert_eq!("34", dec_number_to_string(n));
}

#[test]
fn test_decimal32_from_number() {
  let dn = dec_number_from_u32(32);
  let ds = decimal32_from_number(&dn, &mut dec_context_32());
  assert_eq!("32", dec_single_to_string(&ds));
}

#[test]
fn test_decimal64_to_number() {
  let dd = dec_double_from_string("64", &mut dec_context_64());
  let mut dn = DecNumber::default();
  let n = decimal64_to_number(&dd, &mut dn);
  assert_eq!("64", dec_number_to_string(n));
}

#[test]
fn test_decimal64_from_number() {
  let dn = dec_number_from_u32(64);
  let dd = decimal64_from_number(&dn, &mut dec_context_64());
  assert_eq!("64", dec_double_to_string(&dd));
}

#[test]
fn test_decimal128_to_number() {
  let dq = dec_quad_from_string("128", &mut dec_context_128());
  let mut dn = DecNumber::default();
  let n = decimal128_to_number(&dq, &mut dn);
  assert_eq!("128", dec_number_to_string(n));
}

#[test]
fn test_decimal128_from_number() {
  let dn = dec_number_from_u32(128);
  let dq = decimal128_from_number(&dn, &mut dec_context_128());
  assert_eq!("128", dec_quad_to_string(&dq));
}
