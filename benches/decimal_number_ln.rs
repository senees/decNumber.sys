#![feature(test)]

extern crate test;

use dec_number_sys::*;
use test::Bencher;

#[bench]
fn bench_decimal_number_ln_0001(b: &mut Bencher) {
  let dn = DecimalNumber::from("1e-400");
  b.iter(|| {
    let _ = dn.ln();
  });
}

#[bench]
fn bench_decimal_number_ln_0002(b: &mut Bencher) {
  let dn = DecimalNumber::from("2");
  b.iter(|| {
    let _ = dn.ln();
  });
}

#[bench]
fn bench_decimal_number_ln_0003(b: &mut Bencher) {
  let dn = DecimalNumber::from("0.0086732880815927182997566810334394");
  b.iter(|| {
    let _ = dn.ln();
  });
}
