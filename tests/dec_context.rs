use dec_number_sys::*;

#[test]
fn test_dec_context_decimal_base() {
  let context = dec_context_base(34);
  assert_eq!(34, context.digits);
  assert_eq!(999999999, context.emax);
  assert_eq!(-999999999, context.emin);
  assert_eq!(2, context.round);
  assert_eq!(0, context.traps);
  assert_eq!(0, context.status);
  assert_eq!(0, context.clamp);
}

#[test]
fn test_dec_context_decimal_32_001() {
  let context = dec_context_32();
  assert_eq!(7, context.digits);
  assert_eq!(96, context.emax);
  assert_eq!(-95, context.emin);
  assert_eq!(3, context.round);
  assert_eq!(0, context.traps);
  assert_eq!(0, context.status);
  assert_eq!(1, context.clamp);
}

#[test]
fn test_dec_context_decimal_32_002() {
  let context = dec_context_default(DEC_INIT_DECIMAL32);
  assert_eq!(7, context.digits);
  assert_eq!(96, context.emax);
  assert_eq!(-95, context.emin);
  assert_eq!(3, context.round);
  assert_eq!(0, context.traps);
  assert_eq!(0, context.status);
  assert_eq!(1, context.clamp);
}

#[test]
fn test_dec_context_decimal_64_001() {
  let context = dec_context_64();
  assert_eq!(16, context.digits);
  assert_eq!(384, context.emax);
  assert_eq!(-383, context.emin);
  assert_eq!(3, context.round);
  assert_eq!(0, context.traps);
  assert_eq!(0, context.status);
  assert_eq!(1, context.clamp);
}

#[test]
fn test_dec_context_decimal_64_002() {
  let context = dec_context_default(DEC_INIT_DECIMAL64);
  assert_eq!(16, context.digits);
  assert_eq!(384, context.emax);
  assert_eq!(-383, context.emin);
  assert_eq!(3, context.round);
  assert_eq!(0, context.traps);
  assert_eq!(0, context.status);
  assert_eq!(1, context.clamp);
}

#[test]
fn test_dec_context_decimal_128_001() {
  let context = dec_context_128();
  assert_eq!(34, context.digits);
  assert_eq!(6144, context.emax);
  assert_eq!(-6143, context.emin);
  assert_eq!(3, context.round);
  assert_eq!(0, context.traps);
  assert_eq!(0, context.status);
  assert_eq!(1, context.clamp);
}

#[test]
fn test_dec_context_decimal_128_002() {
  let context = dec_context_default(DEC_INIT_DECIMAL128);
  assert_eq!(34, context.digits);
  assert_eq!(6144, context.emax);
  assert_eq!(-6143, context.emin);
  assert_eq!(3, context.round);
  assert_eq!(0, context.traps);
  assert_eq!(0, context.status);
  assert_eq!(1, context.clamp);
}

#[test]
fn test_dec_context_zero_status() {
  let mut context = dec_context_default(DEC_INIT_DECIMAL128);
  context.status = 0xFFFFFFFF;
  dec_context_zero_status(&mut context);
  assert_eq!(0, context.status);
}

#[test]
#[allow(clippy::redundant_clone)]
fn test_dec_context_clone() {
  let context = dec_context_default(DEC_INIT_DECIMAL128);
  let context1 = context.clone();
  assert_eq!(34, context1.digits);
  assert_eq!(6144, context1.emax);
  assert_eq!(-6143, context1.emin);
  assert_eq!(3, context1.round);
  assert_eq!(0, context1.traps);
  assert_eq!(0, context1.status);
  assert_eq!(1, context1.clamp);
}
