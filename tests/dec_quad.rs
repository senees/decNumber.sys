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

//! [DecQuad] smoke tests.
//!
//! The purpose of smoke tests is to verify if bindings compile properly.

use dec_number_sys::*;

macro_rules! c {
  () => {
    &mut dec_context_128()
  };
}

macro_rules! n {
  ($s:expr) => {
    dec_quad_from_string(stringify!($s), c!())
  };
}

macro_rules! s {
  ($v:expr) => {
    dec_quad_to_string(&$v)
  };
}

#[test]
fn test_dec_quad_abs() {
  assert_eq!("5.751", s!(dec_quad_abs(&n!(-5.751), c!())));
}

#[test]
fn test_dec_quad_add() {
  assert_eq!("9.05", s!(dec_quad_add(&n!(5.75), &n!(3.3), c!())));
}

#[test]
#[rustfmt::skip]
fn test_dec_quad_constants() {
  assert_eq!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", format!("{:?}", n!(0)));
  assert_eq!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", format!("{:?}", DEC_QUAD_ZERO));
  // one
  assert_eq!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 01]", format!("{:?}", n!(1)));
  assert_eq!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 01]", format!("{:?}", DEC_QUAD_ONE));
  // two
  assert_eq!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 02]", format!("{:?}", n!(2)));
  assert_eq!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 02]", format!("{:?}", DEC_QUAD_TWO));
  // ten
  assert_eq!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 10]", format!("{:?}", n!(10)));
  assert_eq!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 10]", format!("{:?}", DEC_QUAD_TEN));
  // hundred
  assert_eq!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 80]", format!("{:?}", n!(100)));
  assert_eq!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 80]", format!("{:?}", DEC_QUAD_HUNDRED));
  // thousand
  assert_eq!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 04 00]", format!("{:?}", n!(1000)));
  assert_eq!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 04 00]", format!("{:?}", DEC_QUAD_THOUSAND));
  // million
  assert_eq!("[22 08 00 00 00 00 00 00 00 00 00 00 00 10 00 00]", format!("{:?}", n!(1000000)));
  assert_eq!("[22 08 00 00 00 00 00 00 00 00 00 00 00 10 00 00]", format!("{:?}", DEC_QUAD_MILLION));
  // billion
  assert_eq!("[22 08 00 00 00 00 00 00 00 00 00 00 40 00 00 00]", format!("{:?}", n!(1000000000)));
  assert_eq!("[22 08 00 00 00 00 00 00 00 00 00 00 40 00 00 00]", format!("{:?}", DEC_QUAD_BILLION));
}

#[test]
#[rustfmt::skip]
fn test_dec_quad_from_bcd() {
  assert_eq!("2366920938463463374607431768211455", s!(dec_quad_from_bcd(&bcd_quad(u128::MAX), 0, false)));
  assert_eq!("18446744073709551615", s!(dec_quad_from_bcd(&bcd_quad(u64::MAX.into()), 0, false)));
  assert_eq!("-18446744073709551615", s!(dec_quad_from_bcd(&bcd_quad(u64::MAX.into()), 0, true)));
  assert_eq!("9223372036854775807", s!(dec_quad_from_bcd(&bcd_quad(i64::MAX.unsigned_abs().into()), 0, false)));
  assert_eq!("-9223372036854775808", s!(dec_quad_from_bcd(&bcd_quad(i64::MIN.unsigned_abs().into()), 0, true)));
  assert_eq!("1844674407.3709551615", s!(dec_quad_from_bcd(&bcd_quad(u64::MAX.into()), -10, false)));
}

#[test]
fn test_dec_quad_reduce() {
  assert_eq!("1.2345678E+11", s!(dec_quad_reduce(&n!(12345678E+4), c!())));
}

#[test]
fn test_dec_quad_scale_b() {
  assert_eq!("1234.5678", s!(dec_quad_scale_b(&n!(12345678), &n!(-4), c!())));
}

#[test]
fn test_dec_quad_to_integral_value() {
  assert_eq!(
    "-0",
    s!(dec_quad_to_integral_value(&n!(-0.75), c!(), DEC_ROUND_CEILING))
  );
}
