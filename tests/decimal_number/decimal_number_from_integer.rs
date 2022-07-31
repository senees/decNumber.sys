use dec_number_sys::DecimalNumber;

/// Utility function to simplify type conversions in test cases.
fn eq(expected: &str, actual: DecimalNumber) {
  assert_eq!(expected, actual.to_string());
}

#[test]
fn decimal_number_from_integer_0001() {
  eq("23", 23_u8.into());
  eq("-23", (-23_i8).into());
}

#[test]
fn decimal_number_from_integer_0002() {
  eq("1023", 1023_u16.into());
  eq("-1023", (-1023_i16).into());
}

#[test]
fn decimal_number_from_integer_0003() {
  eq("1023023", 1023023_u32.into());
  eq("-1023023", (-1023023_i32).into());
}
