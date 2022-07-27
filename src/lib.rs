//!

extern crate libc;

mod bindings;
mod dec_number_c;

pub use bindings::{
  dec_context_default, dec_quad_add, dec_quad_from_string, dec_quad_to_string, ContextKind, DecContext,
};
