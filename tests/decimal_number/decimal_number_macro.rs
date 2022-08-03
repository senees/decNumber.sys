use dec_number_sys::{decimal_number, DecimalNumber};

#[test]
fn decimal_number_macro_0001() {
  let dn = decimal_number!(0.1);
  assert_eq!("0.1", dn.to_string());
}

#[test]
fn decimal_number_macro_0002() {
  let dn = decimal_number!(1.234E+2);
  assert_eq!("123.4", dn.to_string());
}

#[test]
fn decimal_number_macro_0003() {
  let dn = decimal_number!(-23.84756474857);
  assert_eq!("-23.84756474857", dn.to_string());
}

#[test]
fn decimal_number_macro_0004() {
  {
    use dec_number_sys::decimal_number as dec;
    let dn = dec!(-0.23);
    assert_eq!("-0.23", dn.to_string());
  }
}
