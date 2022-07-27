//!

use crate::dec_conversion_c::*;
use crate::dec_number::DecNumber;
use crate::dec_quad::DecQuad;
use crate::DecContext;

///
pub fn decimal128_to_number(dq: &DecQuad, dn: &mut DecNumber) {
  unsafe {
    decimal128ToNumber(dq, dn);
  }
}

///
pub fn decimal128_from_number(dn: &DecNumber, ctx: &mut DecContext) -> DecQuad {
  let mut result = DecQuad::default();
  unsafe {
    decimal128FromNumber(&mut result, dn, ctx);
  }
  result
}
