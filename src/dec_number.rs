//!

#[repr(C)]
#[derive(Default)]
pub struct DecNumber {
  digits: i32,
  exponent: i32,
  bits: u8,
  lsu: [u16; 12],
}
