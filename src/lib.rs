//!

extern crate libc;

mod dec_common;
mod dec_context;
mod dec_context_c;
mod dec_conversion;
mod dec_conversion_c;
mod dec_number;
mod dec_number_c;
mod dec_quad;
mod dec_quad_c;
mod dec_single;
mod dec_single_c;

pub use dec_quad::{
  dec_quad_add, dec_quad_from_i32, dec_quad_from_string, dec_quad_from_u32, dec_quad_rescale, dec_quad_to_string,
};

pub use dec_single::{dec_single_add, dec_single_get_zero, dec_single_zero};

pub use dec_context::{dec_context_default, ContextKind, DecContext};

pub use dec_conversion::{decimal128_from_number, decimal128_to_number};
