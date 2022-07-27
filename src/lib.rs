//!

extern crate libc;

mod bindings;
mod dec_number_c;

pub use bindings::{dec_context_default, ContextKind, DecContext};
