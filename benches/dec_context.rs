#![feature(test)]

extern crate test;

use dec_number_sys::*;
use test::Bencher;

#[bench]
fn bench_dec_context_base(b: &mut Bencher) {
  b.iter(|| {
    let _ = dec_context_default(ContextKind::Base);
  });
}

#[bench]
fn bench_dec_context_32(b: &mut Bencher) {
  b.iter(|| {
    let _ = dec_context_default(ContextKind::Decimal32);
  });
}

#[bench]
fn bench_dec_context_64(b: &mut Bencher) {
  b.iter(|| {
    let _ = dec_context_default(ContextKind::Decimal64);
  });
}

#[bench]
fn bench_dec_context_128(b: &mut Bencher) {
  b.iter(|| {
    let _ = dec_context_default(ContextKind::Decimal128);
  });
}
