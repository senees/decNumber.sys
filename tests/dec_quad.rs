//! [DecQuad] smoke tests.

use dec_number_sys::*;

#[test]
fn test_dec_quad_abs() {
  let dc = &mut dec_context_128();
  dec_context_zero_status(dc);
  let dq = &dec_quad_from_string("-5.751", dc);
  assert_eq!(0, dc.status);
  let dq_res = &dec_quad_abs(dq, dc);
  assert_eq!(0, dc.status);
  assert_eq!("5.751", dec_quad_to_string(dq_res));
}

#[test]
fn test_dec_quad_add() {
  let dc = &mut dec_context_128();
  dec_context_zero_status(dc);
  let dq1 = &dec_quad_from_string("5.75", dc);
  assert_eq!(0, dc.status);
  dec_context_zero_status(dc);
  let dq2 = &dec_quad_from_string("3.3", dc);
  assert_eq!(0, dc.status);
  dec_context_zero_status(dc);
  let dq_res = &dec_quad_add(dq1, dq2, dc);
  assert_eq!(0, dc.status);
  assert_eq!("9.05", dec_quad_to_string(dq_res));
}

#[test]
#[rustfmt::skip]
fn test_dec_quad_constants() {
  let dc = &mut dec_context_128();
  // zero
  dec_context_zero_status(dc);
  let dq_zero = &dec_quad_from_string("0", dc);
  assert_eq!(0, dc.status);
  assert_eq!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", format!("{:?}", dq_zero));
  assert_eq!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", format!("{:?}", DEC_QUAD_ZERO));
  // one
  dec_context_zero_status(dc);
  let dq_one = &dec_quad_from_string("1", dc);
  assert_eq!(0, dc.status);
  assert_eq!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 01]", format!("{:?}", dq_one));
  assert_eq!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 01]", format!("{:?}", DEC_QUAD_ONE));
  // two
  dec_context_zero_status(dc);
  let dq_two = &dec_quad_from_string("2", dc);
  assert_eq!(0, dc.status);
  assert_eq!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 02]", format!("{:?}", dq_two));
  assert_eq!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 02]", format!("{:?}", DEC_QUAD_TWO));
  // ten
  dec_context_zero_status(dc);
  let dq_ten = &dec_quad_from_string("10", dc);
  assert_eq!(0, dc.status);
  assert_eq!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 10]", format!("{:?}", dq_ten));
  assert_eq!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 10]", format!("{:?}", DEC_QUAD_TEN));
  // hundred
  dec_context_zero_status(dc);
  let dq_hundred = &dec_quad_from_string("100", dc);
  assert_eq!(0, dc.status);
  assert_eq!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 80]", format!("{:?}", dq_hundred));
  assert_eq!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 80]", format!("{:?}", DEC_QUAD_HUNDRED));
  // thousand
  dec_context_zero_status(dc);
  let dq_thousand = &dec_quad_from_string("1000", dc);
  assert_eq!(0, dc.status);
  assert_eq!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 04 00]", format!("{:?}", dq_thousand));
  assert_eq!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 04 00]", format!("{:?}", DEC_QUAD_THOUSAND));
  // million
  dec_context_zero_status(dc);
  let dq_million = &dec_quad_from_string("1000000", dc);
  assert_eq!(0, dc.status);
  assert_eq!("[22 08 00 00 00 00 00 00 00 00 00 00 00 10 00 00]", format!("{:?}", dq_million));
  assert_eq!("[22 08 00 00 00 00 00 00 00 00 00 00 00 10 00 00]", format!("{:?}", DEC_QUAD_MILLION));
  // billion
  dec_context_zero_status(dc);
  let dq_billion = &dec_quad_from_string("1000000000", dc);
  assert_eq!(0, dc.status);
  assert_eq!("[22 08 00 00 00 00 00 00 00 00 00 00 40 00 00 00]", format!("{:?}", dq_billion));
  assert_eq!("[22 08 00 00 00 00 00 00 00 00 00 00 40 00 00 00]", format!("{:?}", DEC_QUAD_BILLION));
}
