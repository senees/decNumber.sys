use dec_number_sys::DecimalNumber;

/// Utility function to simplify test cases - no rounding.
fn eq(expected: &str, input: &str) {
  let actual = DecimalNumber::from(input).ln();
  assert_eq!(expected, actual.to_string());
}

/// Utility function to simplify test cases - with rounding.
fn eqr(expected: &str, input: &str, dp: i32) {
  let actual = DecimalNumber::from(input).ln();
  assert_eq!(expected, actual.round_dp(dp).to_string());
}

#[test]
fn decimal_number_ln_0001() {
  eq("-Infinity", "0");
}

#[test]
fn decimal_number_ln_0002() {
  eq("0", "1");
}

#[test]
fn decimal_number_ln_0003() {
  eqr("1.00000000", "2.71828183", 8);
}

#[test]
fn decimal_number_ln_0004() {
  eqr("2.30258509", "10", 8);
}

#[test]
fn decimal_number_ln_0005() {
  eq("Infinity", "+Infinity");
}

#[test]
fn decimal_number_ln_0006() {
  eqr("-20.72326583694641", "1E-9", 14);
}

#[test]
fn decimal_number_ln_0007() {
  eqr("-7.264430222920869", "0.0007", 15);
}

#[test]
fn decimal_number_ln_0008() {
  eq("0.6931471805599453094172321214581766", "2");
}

#[test]
fn decimal_number_ln_0009() {
  eqr("-921.0340371976183", "1e-400", 13);
}

#[test]
fn decimal_number_ln_0010() {
  eq(
    "-4.747507311920844752486938187973721",
    "0.0086732880815927182997566810334394",
  );
}
