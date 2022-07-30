//!

extern crate libc;

mod dec_common;
mod dec_context;
mod dec_context_c;
mod dec_conversion;
mod dec_conversion_c;
mod dec_double;
mod dec_double_c;
mod dec_number;
mod dec_number_c;
mod dec_quad;
mod dec_quad_c;
mod dec_single;
mod dec_single_c;
mod decimal128;
mod decimal32;
mod decimal64;
mod decimal_number;

pub use dec_context::{dec_context_default, ContextKind, DecContext};
pub use dec_conversion::{
  decimal128_from_number, decimal128_to_number, decimal32_from_number, decimal32_to_number, decimal64_from_number,
  decimal64_to_number,
};
pub use dec_double::{dec_double_add, dec_double_zero};
pub use dec_number::{
  dec_number_add, dec_number_divide, dec_number_exp, dec_number_from_string, dec_number_ln, dec_number_multiply,
  dec_number_subtract, dec_number_to_string, dec_number_zero,
};
pub use dec_quad::{
  dec_quad_add, dec_quad_from_i32, dec_quad_from_string, dec_quad_from_u32, dec_quad_rescale, dec_quad_to_string,
  dec_quad_zero,
};
pub use dec_single::{dec_single_from_string, dec_single_to_string, dec_single_zero};
pub use decimal128::Decimal128;
pub use decimal_number::DecimalNumber;
