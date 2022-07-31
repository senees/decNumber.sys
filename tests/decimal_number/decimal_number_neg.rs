use dec_number_sys::DecimalNumber;

#[test]
fn decimal_number_neg_0001() {
  let dn = DecimalNumber::from("0");
  assert_eq!("0", (-dn).to_string());
}

#[test]
fn decimal_number_neg_0002() {
  let dn = DecimalNumber::from("0.123456");
  assert_eq!("-0.123456", (-dn).to_string());
}

#[test]
fn decimal_number_neg_0003() {
  let dn = DecimalNumber::from("-0.123456");
  assert_eq!("0.123456", (-dn).to_string());
}
