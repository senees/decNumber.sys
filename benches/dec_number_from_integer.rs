#![feature(test)]

extern crate test;

use dec_number_sys::*;
use test::Bencher;

#[bench]
fn bench_dec_number_from_integer_u64(b: &mut Bencher) {
  b.iter(|| {
    let _ = dec_number_from_u64(18_446_744_073_709_551_615_u64);
  });
}

#[bench]
fn bench_dec_number_from_integer_i64(b: &mut Bencher) {
  let dc = &mut dec_context_default(ContextKind::Decimal128);
  b.iter(|| {
    let _ = dec_number_from_i64(-9_223_372_036_854_775_808_i64, dc);
  });
}
