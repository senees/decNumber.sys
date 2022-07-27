use dec_number_sys::{
  dec_context_default, dec_quad_add, dec_quad_from_i32, dec_quad_from_string, dec_quad_rescale, dec_quad_to_string,
  ContextKind,
};

const P8: Option<i32> = Some(8);

fn add(lhs: &str, rhs: &str, expected: &str, precision: Option<i32>) {
  let ctx = &mut dec_context_default(ContextKind::Decimal128);
  let dql = &dec_quad_from_string(lhs, ctx);
  let dqr = &dec_quad_from_string(rhs, ctx);
  let actual = &dec_quad_add(dql, dqr, ctx);
  if let Some(p) = precision {
    let precision = &dec_quad_from_i32(-p);
    let rescaled = &dec_quad_rescale(actual, precision, ctx);
    assert_eq!(expected, dec_quad_to_string(rescaled));
  } else {
    assert_eq!(expected, dec_quad_to_string(actual));
  }
}

#[test]
fn dec_quad_add_0001() {
  add("1", "1", "2", None);
}

#[test]
fn dec_quad_add_0002() {
  add("2", "3", "5", None);
}

#[test]
fn dec_quad_add_0003() {
  add("5.75", "3.3", "9.05", None);
}

#[test]
fn dec_quad_add_0009() {
  add("1.23456789", "1.00000000", "2.23456789", None);
}

#[test]
fn dec_quad_add_0011() {
  add("0.4444444444", "0.5555555555", "1.00000000", P8);
}
