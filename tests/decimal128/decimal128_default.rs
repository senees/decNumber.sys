use dec_number_sys::Decimal128;

#[test]
fn decimal128_default_0001() {
  let d = Decimal128::default();
  assert_eq!("0", d.to_string());
}
