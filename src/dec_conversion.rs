//!

use crate::dec_conversion_c::*;
use crate::dec_double::DecDouble;
use crate::dec_number::DecNumber;
use crate::dec_quad::DecQuad;
use crate::dec_single::DecSingle;
use crate::DecContext;

///
pub fn decimal32_to_number(d: &DecSingle, n: &mut DecNumber) {
  unsafe {
    decimal32ToNumber(d, n);
  }
}

///
pub fn decimal32_from_number(n: &DecNumber, ctx: &mut DecContext) -> DecSingle {
  let mut res = DecSingle::default();
  unsafe {
    decimal32FromNumber(&mut res, n, ctx);
  }
  res
}

///
pub fn decimal64_to_number(d: &DecDouble, n: &mut DecNumber) {
  unsafe {
    decimal64ToNumber(d, n);
  }
}

///
pub fn decimal64_from_number(n: &DecNumber, ctx: &mut DecContext) -> DecDouble {
  let mut res = DecDouble::default();
  unsafe {
    decimal64FromNumber(&mut res, n, ctx);
  }
  res
}

///
pub fn decimal128_to_number(d: &DecQuad, n: &mut DecNumber) {
  unsafe {
    decimal128ToNumber(d, n);
  }
}

///
pub fn decimal128_from_number(n: &DecNumber, ctx: &mut DecContext) -> DecQuad {
  let mut res = DecQuad::default();
  unsafe {
    decimal128FromNumber(&mut res, n, ctx);
  }
  res
}
