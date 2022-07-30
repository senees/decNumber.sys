#![feature(test)]

extern crate test;

use dec_number_sys::*;
use test::Bencher;

#[bench]
fn bench_decimal_number_add_0001(b: &mut Bencher) {
  let dnl = DecimalNumber::from("0.1");
  let dnr = DecimalNumber::from("0.2");
  b.iter(|| {
    let _ = dnl + dnr;
  });
}

#[bench]
fn bench_decimal_number_add_0002(b: &mut Bencher) {
  let dnl = DecimalNumber::from("0.111111");
  let dnr = DecimalNumber::from("0.222");
  b.iter(|| {
    let _ = dnl + dnr;
  });
}
