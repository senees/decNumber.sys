#![feature(test)]

extern crate test;

use dec_number_sys::*;
use test::Bencher;

#[bench]
fn bench_dec_number_add_0001(b: &mut Bencher) {
  let ctx = &mut dec_context_default(ContextKind::Base(34));
  let dnl = &dec_number_from_string("0.1", ctx);
  let dnr = &dec_number_from_string("0.2", ctx);
  b.iter(|| {
    let _ = dec_number_add(dnl, dnr, ctx);
  });
}

#[bench]
fn bench_dec_number_add_0002(b: &mut Bencher) {
  let ctx = &mut dec_context_default(ContextKind::Base(34));
  let dnl = &dec_number_from_string("0.111111", ctx);
  let dnr = &dec_number_from_string("0.222", ctx);
  b.iter(|| {
    let _ = dec_number_add(dnl, dnr, ctx);
  });
}
