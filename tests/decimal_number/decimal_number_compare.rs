use dec_number_sys::DecimalNumber;

#[test]
fn decimal_number_compare_0001() {
  let dn1 = DecimalNumber::from("0.1");
  let dn2 = DecimalNumber::from("0.2");
  assert!(dn1 < dn2);
  assert!(!(dn1 >= dn2));
}

#[test]
fn decimal_number_compare_0002() {
  let dn1 = DecimalNumber::from("0.1");
  let dn2 = DecimalNumber::from("0.1");
  assert!(dn1 <= dn2);
  assert!(!(dn1 > dn2));
}

#[test]
fn decimal_number_compare_0003() {
  let dn1 = DecimalNumber::from("0.15");
  let dn2 = DecimalNumber::from("0.1");
  assert!(dn1 > dn2);
  assert!(!(dn1 <= dn2));
}

#[test]
fn decimal_number_compare_0004() {
  let dn1 = DecimalNumber::from("0.15");
  let dn2 = DecimalNumber::from("0.15");
  assert!(dn1 >= dn2);
  assert!(!(dn1 < dn2));
}

#[test]
fn decimal_number_compare_0005() {
  let dn1 = DecimalNumber::from("0.15");
  let dn2 = DecimalNumber::from("0.15");
  assert!(dn1 == dn2);
  assert!(!(dn1 != dn2));
}

#[test]
fn decimal_number_compare_0006() {
  let dn1 = DecimalNumber::from("0.15");
  let dn2 = DecimalNumber::from("0.16");
  assert!(dn1 != dn2);
  assert!(!(dn1 == dn2));
}
