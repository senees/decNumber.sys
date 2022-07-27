use dec_number_sys::{dec_context_default, dec_quad_add, dec_quad_from_string, dec_quad_to_string, ContextKind};

fn add(lhs: &str, rhs: &str, expected: &str) {
  let ctx = &mut dec_context_default(ContextKind::Decimal128);
  let dql = &dec_quad_from_string(lhs, ctx);
  let dqr = &dec_quad_from_string(rhs, ctx);
  let actual = dec_quad_add(dql, dqr, ctx);
  assert_eq!(expected, dec_quad_to_string(&actual));
}

#[test]
fn dec_quad_add_0001() {
  add("1", "1", "2");
}

#[test]
fn dec_quad_add_0002() {
  add("2", "3", "5");
}

#[test]
fn dec_quad_add_0003() {
  add("5.75", "3.3", "9.05");
}

#[test]
fn dec_quad_add_0009() {
  add("1.23456789", "1.00000000", "2.23456789");
}

#[ignore]
#[test]
fn dec_quad_add_0011() {
  add("0.4444444444", "0.5555555555", "1.00000000");
}
