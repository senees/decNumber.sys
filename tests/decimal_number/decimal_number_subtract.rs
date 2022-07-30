use dec_number_sys::DecimalNumber;

#[test]
fn decimal_number_divide_0001() {
  let dn1 = DecimalNumber::from("1");
  let dn2 = DecimalNumber::from("2");
  assert_eq!("0.5", (dn1 / dn2).to_string());
}

#[test]
fn decimal_number_divide_0002() {
  let mut dn1 = DecimalNumber::from("1");
  let dn2 = DecimalNumber::from("2");
  dn1 /= dn2;
  assert_eq!("0.5", dn1.to_string());
}

#[test]
fn decimal_number_divide_0003() {
  let dn1 = DecimalNumber::from("1");
  let dn2 = DecimalNumber::from("3");
  assert_eq!("0.3333333333333333333333333333333333", (dn1 / dn2).to_string());
}

#[test]
fn decimal_number_divide_0004() {
  let mut dn1 = DecimalNumber::from("1");
  let dn2 = DecimalNumber::from("3");
  dn1 /= dn2;
  assert_eq!("0.3333333333333333333333333333333333", dn1.to_string());
}
