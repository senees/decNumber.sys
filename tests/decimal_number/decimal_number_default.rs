use dec_number_sys::DecimalNumber;

#[test]
fn decimal_number_default() {
  assert_eq!("0", DecimalNumber::default().to_string());
}

#[test]
fn decimal_number_zero() {
  assert_eq!("0", DecimalNumber::zero().to_string());
  assert!(DecimalNumber::zero().is_zero());
}

#[test]
fn decimal_number_one() {
  assert_eq!("1", DecimalNumber::one().to_string());
  assert!(!DecimalNumber::one().is_zero());
}

#[test]
fn decimal_number_two() {
  assert_eq!("2", DecimalNumber::two().to_string());
}

#[test]
fn decimal_number_ten() {
  assert_eq!("10", DecimalNumber::ten().to_string());
}

#[test]
fn decimal_number_one_hundred() {
  assert_eq!("100", DecimalNumber::one_hundred().to_string());
}

#[test]
fn decimal_number_one_thousand() {
  assert_eq!("1000", DecimalNumber::one_thousand().to_string());
}
