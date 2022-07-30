use dec_number_sys::DecimalNumber;

#[test]
fn decimal_number_add_0001() {
  let dn1 = DecimalNumber::from("0.1");
  let dn2 = DecimalNumber::from("0.2");
  assert_eq!("0.3", (dn1 + dn2).to_string());
}

#[test]
fn decimal_number_add_0002() {
  let mut dn1 = DecimalNumber::from("0.1");
  let dn2 = DecimalNumber::from("0.2");
  dn1 += dn2;
  assert_eq!("0.3", dn1.to_string());
}

#[test]
fn decimal_number_add_0003() {
  let dn1 = DecimalNumber::from("0.111111");
  let dn2 = DecimalNumber::from("0.222");
  assert_eq!("0.333111", (dn1 + dn2).to_string());
}

#[test]
fn decimal_number_add_0004() {
  let mut dn1 = DecimalNumber::from("0.111111");
  let dn2 = DecimalNumber::from("0.222");
  dn1 += dn2;
  assert_eq!("0.333111", dn1.to_string());
}

#[test]
fn decimal_number_add_0005() {
  let dn1 = DecimalNumber::from("0.1000000000");
  let dn2 = DecimalNumber::from("0.20000");
  assert_eq!("0.3", (dn1 + dn2).to_string());
}
