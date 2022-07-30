use dec_number_sys::DecimalNumber;

#[test]
fn decimal_number_round_dp_0001() {
  assert_eq!("0.1", DecimalNumber::from("0.1").round_dp(1).to_string());
}

#[test]
fn decimal_number_round_dp_0002() {
  assert_eq!("0.2", DecimalNumber::from("0.15").round_dp(1).to_string());
}

#[test]
fn decimal_number_round_dp_0003() {
  assert_eq!("2E+1", DecimalNumber::from("15").round_dp(-1).to_string());
}
