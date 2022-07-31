use dec_number_sys::DecimalNumber;

#[test]
fn decimal_number_default() {
  assert_eq!("0", DecimalNumber::default().to_string());
  assert_eq!("0", format!("{}", DecimalNumber::default()));
  assert_eq!("0", format!("{:?}", DecimalNumber::default()));
}

#[test]
fn decimal_number_zero() {
  assert_eq!("0", DecimalNumber::zero().to_string());
  assert_eq!("0", format!("{}", DecimalNumber::zero()));
  assert_eq!("0", format!("{:?}", DecimalNumber::zero()));
  assert!(DecimalNumber::zero().is_zero());
}

#[test]
fn decimal_number_one() {
  assert_eq!("1", DecimalNumber::one().to_string());
  assert_eq!("1", format!("{}", DecimalNumber::one()));
  assert_eq!("1", format!("{:?}", DecimalNumber::one()));
  assert!(!DecimalNumber::one().is_zero());
}

#[test]
fn decimal_number_two() {
  assert_eq!("2", DecimalNumber::two().to_string());
  assert_eq!("2", format!("{}", DecimalNumber::two()));
  assert_eq!("2", format!("{:?}", DecimalNumber::two()));
}

#[test]
fn decimal_number_ten() {
  assert_eq!("10", DecimalNumber::ten().to_string());
  assert_eq!("10", format!("{}", DecimalNumber::ten()));
  assert_eq!("10", format!("{:?}", DecimalNumber::ten()));
}

#[test]
fn decimal_number_one_hundred() {
  assert_eq!("100", DecimalNumber::one_hundred().to_string());
  assert_eq!("100", format!("{}", DecimalNumber::one_hundred()));
  assert_eq!("100", format!("{:?}", DecimalNumber::one_hundred()));
}

#[test]
fn decimal_number_one_thousand() {
  assert_eq!("1000", DecimalNumber::one_thousand().to_string());
  assert_eq!("1000", format!("{}", DecimalNumber::one_thousand()));
  assert_eq!("1000", format!("{:?}", DecimalNumber::one_thousand()));
}

#[test]
fn decimal_number_scale() {
  assert_eq!("123.45", DecimalNumber::new(12345, 2).to_string());
  assert_eq!("1.2345E+6", DecimalNumber::new(12345, -2).to_string());
}
