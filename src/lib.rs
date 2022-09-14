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

//! Rust bindings for **The decNumber C library** by IBM Fellow Mike Cowlishaw.

extern crate libc;

mod dec_context;
mod dec_context_c;
mod dec_conversion;
mod dec_conversion_c;
mod dec_double;
mod dec_double_c;
mod dec_number;
mod dec_number_c;
mod dec_quad;
mod dec_quad_c;
mod dec_single;
mod dec_single_c;

pub use dec_context::*;
pub use dec_context_c::*;
pub use dec_conversion::*;
pub use dec_conversion_c::*;
pub use dec_double::*;
pub use dec_double_c::*;
pub use dec_number::*;
pub use dec_number_c::*;
pub use dec_quad::*;
pub use dec_quad_c::*;
pub use dec_single::*;
pub use dec_single_c::*;
