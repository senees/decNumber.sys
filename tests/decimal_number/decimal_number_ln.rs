use dec_number_sys::DecimalNumber;

#[test]
fn decimal_number_ln_0001() {
  let dn = DecimalNumber::from("0");
  assert_eq!("-Infinity", dn.ln().to_string());
}

#[test]
fn decimal_number_ln_0002() {
  let dn = DecimalNumber::from("1");
  assert_eq!("0", dn.ln().to_string());
}

#[test]
fn decimal_number_ln_0003() {
  let dn = DecimalNumber::from("2.718281828459045");
  assert_eq!("0.9999999999999999134157889710887612", dn.ln().to_string());
}

#[test]
fn decimal_number_ln_0004() {
  let dn = DecimalNumber::from("1");
  assert_eq!("1", dn.ln().exp().to_string());
}
