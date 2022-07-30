use dec_number_sys::DecimalNumber;

#[test]
fn decimal_number_from_string_0001() {
  assert_eq!("0", DecimalNumber::default().to_string());
}

#[test]
fn decimal_number_from_string_0002() {
  assert_eq!("0", DecimalNumber::from("0").to_string());
}

#[test]
fn decimal_number_from_string_0003() {
  assert_eq!("1", DecimalNumber::from("1").to_string());
}

#[test]
fn decimal_number_from_string_0004() {
  assert_eq!("-0", DecimalNumber::from("-0").to_string());
}

#[test]
fn decimal_number_from_string_0005() {
  assert_eq!("-1", DecimalNumber::from("-1").to_string());
}

#[test]
fn decimal_number_from_string_0006() {
  assert_eq!("10", DecimalNumber::from("10").to_string());
}

#[test]
fn decimal_number_from_string_0007() {
  assert_eq!("100", DecimalNumber::from("100").to_string());
}

#[test]
fn decimal_number_from_string_0008() {
  assert_eq!("1000", DecimalNumber::from("1000").to_string());
}
