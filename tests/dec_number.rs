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

//! [DecNumber] smoke tests.
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
    dec_number_from_string(stringify!($s), c!())
  };
}

macro_rules! s {
  ($v:expr) => {
    dec_number_to_string(&$v)
  };
}

#[test]
fn test_number_add() {
  assert_eq!("1.999", s!(dec_number_add(&n!(1.234), &n!(0.765), c!())));
}

#[test]
fn test_number_exp() {
  assert_eq!("7.389056098930650227230427460575008", s!(dec_number_exp(&n!(2), c!())));
}

#[test]
fn test_number_is_zero() {
  assert!(dec_number_is_zero(&n!(0)));
  assert!(!dec_number_is_zero(&n!(0.1)));
  assert!(!dec_number_is_zero(&n!(-0.1)));
}

#[test]
fn test_number_power() {
  assert_eq!("8", s!(dec_number_power(&n!(2), &n!(3), c!())));
}

#[test]
fn test_number_quantize() {
  assert_eq!("123.5", s!(dec_number_quantize(&n!(123.456), &n!(10.0), c!())));
}

#[test]
fn test_number_square_root() {
  assert_eq!(
    "1.414213562373095048801688724209698",
    s!(dec_number_square_root(&n!(2), c!()))
  );
}

#[test]
fn test_number_rescale() {
  assert_eq!("123.5", s!(dec_number_rescale(&n!(123.456), &n!(-1), c!())));
}
