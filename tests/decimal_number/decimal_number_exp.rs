use dec_number_sys::DecimalNumber;

#[test]
fn decimal_number_exp_0001() {
  let dn = DecimalNumber::from("0");
  assert_eq!("1", dn.exp().to_string());
}

#[test]
fn decimal_number_exp_0002() {
  let dn = DecimalNumber::from("1");
  assert_eq!("2.718281828459045235360287471352662", dn.exp().to_string());
}

#[test]
fn decimal_number_exp_0003() {
  let dn = DecimalNumber::from("1");
  assert_eq!("0.9999999999999999999999999999999998", dn.exp().ln().to_string());
}
