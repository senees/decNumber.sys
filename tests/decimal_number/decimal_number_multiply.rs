use dec_number_sys::DecimalNumber;

#[test]
fn decimal_number_multiply_0001() {
  let dn1 = DecimalNumber::from("2");
  let dn2 = DecimalNumber::from("2");
  assert_eq!("4", (dn1 * dn2).to_string());
}

#[test]
fn decimal_number_multiply_0002() {
  let mut dn1 = DecimalNumber::from("2");
  let dn2 = DecimalNumber::from("2");
  dn1 *= dn2;
  assert_eq!("4", dn1.to_string());
}

#[test]
fn decimal_number_multiply_0003() {
  let dn1 = DecimalNumber::from("5.09");
  let dn2 = DecimalNumber::from("7.1");
  assert_eq!("36.139", (dn1 * dn2).to_string());
}

#[test]
fn decimal_number_multiply_0004() {
  let mut dn1 = DecimalNumber::from("5.09");
  let dn2 = DecimalNumber::from("7.1");
  dn1 *= dn2;
  assert_eq!("36.139", dn1.to_string());
}
