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

#![feature(test)]

extern crate test;

use dec_number_sys::*;
use test::Bencher;

#[bench]
fn bench_dec_quad_init_0001(b: &mut Bencher) {
  b.iter(|| {
    let _ = DecQuad::default();
  });
}

#[bench]
fn bench_dec_quad_init_0002(b: &mut Bencher) {
  b.iter(|| {
    let _ = DEC_QUAD_ZERO;
  });
}

#[bench]
fn bench_dec_quad_init_0003(b: &mut Bencher) {
  b.iter(|| {
    let _ = DEC_QUAD_ONE;
  });
}

#[bench]
fn bench_dec_quad_init_0004(b: &mut Bencher) {
  b.iter(|| {
    let _ = DEC_QUAD_TWO;
  });
}

#[bench]
fn bench_dec_quad_init_0005(b: &mut Bencher) {
  b.iter(|| {
    let _ = DEC_QUAD_TEN;
  });
}

#[bench]
fn bench_dec_quad_init_0006(b: &mut Bencher) {
  b.iter(|| {
    let _ = DEC_QUAD_HUNDRED;
  });
}

#[bench]
fn bench_dec_quad_init_0007(b: &mut Bencher) {
  b.iter(|| {
    let _ = DEC_QUAD_THOUSAND;
  });
}

#[bench]
fn bench_dec_quad_init_0008(b: &mut Bencher) {
  b.iter(|| {
    let _ = DEC_QUAD_MILLION;
  });
}

#[bench]
fn bench_dec_quad_init_0009(b: &mut Bencher) {
  b.iter(|| {
    let _ = DEC_QUAD_BILLION;
  });
}
