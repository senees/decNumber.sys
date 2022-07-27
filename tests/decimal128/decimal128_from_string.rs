use dec_number_sys::Decimal128;

#[test]
fn decimal128_from_string_0001() {
  assert_eq!("0", Decimal128::default().to_string());
}

#[test]
fn decimal128_from_string_0002() {
  assert_eq!("1", Decimal128::from("1").to_string());
}
