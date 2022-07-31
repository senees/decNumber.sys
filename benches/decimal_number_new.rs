#![feature(test)]

extern crate test;

use dec_number_sys::*;
use test::Bencher;

#[bench]
fn bench_decimal_number_new(b: &mut Bencher) {
  b.iter(|| {
    let _ = DecimalNumber::new(12345, 2);
  });
}
