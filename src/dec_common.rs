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

//! Common definitions.

/// Sign mask.
pub const DEC_FLOAT_SIGN: i32 = -2147483648; // 0x80000000

/// Maximum precision (digits) for 128-bit decimals.
pub const DEC_QUAD_PMAX: usize = 34;

/// Round towards +Infinity.
pub const DEC_ROUND_CEILING: u32 = 0;

/// Round away from 0.
pub const DEC_ROUND_UP: u32 = 1;

/// 0.5 rounds up.
pub const DEC_ROUND_HALF_UP: u32 = 2;

/// 0.5 rounds to nearest even.
pub const DEC_ROUND_HALF_EVEN: u32 = 3;

/// 0.5 rounds down.
pub const DEC_ROUND_HALF_DOWN: u32 = 4;

/// Round towards 0 (truncate).
pub const DEC_ROUND_DOWN: u32 = 5;

/// Round towards -Infinity.
pub const DEC_ROUND_FLOOR: u32 = 6;

/// Round for reround.
pub const DEC_ROUND_05UP: u32 = 7;

/// Converts unsigned integer into BCD representation.
pub fn bcd(n: u128) -> Vec<u8> {
  let mut v = n;
  let mut digits = Vec::<u8>::with_capacity(60);
  loop {
    digits.push((v % 10) as u8);
    v /= 10;
    if v == 0 {
      break;
    }
  }
  digits.reverse();
  digits
}

/// Converts unsigned integer into BCD representation
/// with maximum digits that fit into [DecQuad](crate::DecQuad).
pub fn bcd_quad(n: u128) -> [u8; DEC_QUAD_PMAX] {
  let mut v = n;
  let mut digits = [0; DEC_QUAD_PMAX];
  let mut index = digits.len() - 1;
  loop {
    digits[index] = (v % 10) as u8;
    v /= 10;
    if index == 0 || v == 0 {
      break;
    }
    index -= 1;
  }
  digits
}
