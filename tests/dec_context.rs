use dec_number_sys::{dec_context_128, dec_context_32, dec_context_64, dec_context_base};

#[test]
fn dec_context_decimal_base() {
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
fn dec_context_decimal_32() {
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
fn dec_context_decimal_64() {
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
fn dec_context_decimal_128() {
  let context = dec_context_128();
  assert_eq!(34, context.digits);
  assert_eq!(6144, context.emax);
  assert_eq!(-6143, context.emin);
  assert_eq!(3, context.round);
  assert_eq!(0, context.traps);
  assert_eq!(0, context.status);
  assert_eq!(1, context.clamp);
}
