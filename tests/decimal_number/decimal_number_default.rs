use dec_number_sys::DecimalNumber;

#[test]
fn decimal128_default_0001() {
  let d = DecimalNumber::default();
  assert_eq!("0", d.to_string());
}
