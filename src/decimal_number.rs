use crate::dec_context::*;
use crate::dec_number::*;
use crate::ContextKind::Decimal128;
use std::cmp::Ordering;
use std::convert::Infallible;
use std::str::FromStr;

#[macro_export]
macro_rules! decnum {
  ($e:expr) => {{
    DecimalNumber::from(stringify!($e))
  }};
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DecimalNumber(DecNumber);

impl Default for DecimalNumber {
  /// The default value of [DecimalNumber] is zero.
  fn default() -> Self {
    Self(DecNumber::default())
  }
}

impl std::fmt::Debug for DecimalNumber {
  /// Converts [DecimalNumber] into string in debug mode.
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", dec_number_to_string(&self.0))
  }
}

impl std::fmt::Display for DecimalNumber {
  /// Converts [DecimalNumber] into string.
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", dec_number_to_string(&self.0))
  }
}

impl DecimalNumber {
  ///
  fn default_context() -> DecContext {
    dec_context_default(Decimal128)
  }
  ///
  pub fn new(n: i64, s: i32) -> Self {
    let ctx = &mut Self::default_context();
    let base = &dec_number_from_i64(n, ctx);
    let scale = &dec_number_from_i32(-s);
    Self(dec_number_scale_b(base, scale, ctx))
  }
  ///
  pub fn zero() -> Self {
    Self(DecNumber::zero())
  }
  ///
  pub fn is_zero(&self) -> bool {
    dec_number_is_zero(&self.0)
  }
  ///
  pub fn one() -> Self {
    Self(DecNumber::one())
  }
  ///
  pub fn two() -> Self {
    Self(DecNumber::two())
  }
  ///
  pub fn ten() -> Self {
    Self(DecNumber::ten())
  }
  ///
  pub fn one_hundred() -> Self {
    Self(DecNumber::one_hundred())
  }
  ///
  pub fn one_thousand() -> Self {
    Self(DecNumber::one_thousand())
  }
  ///
  pub fn ln(&self) -> DecimalNumber {
    let mut ctx = Self::default_context();
    Self(dec_number_ln(&self.0, &mut ctx))
  }
  ///
  pub fn exp(&self) -> DecimalNumber {
    let mut ctx = Self::default_context();
    Self(dec_number_exp(&self.0, &mut ctx))
  }
  ///
  pub fn round_dp(&self, dp: i32) -> Self {
    let mut ctx = Self::default_context();
    Self(dec_number_rescale(
      &self.0,
      &dec_number_minus(&dec_number_from_i32(dp), &mut ctx),
      &mut ctx,
    ))
  }
}

impl std::ops::Neg for DecimalNumber {
  type Output = Self;
  fn neg(self) -> Self::Output {
    let ctx = &mut Self::default_context();
    Self(dec_number_minus(&self.0, ctx))
  }
}

impl std::ops::Add<DecimalNumber> for DecimalNumber {
  type Output = Self;
  ///
  fn add(self, rhs: Self) -> Self::Output {
    let mut ctx = Self::default_context();
    Self(dec_number_reduce(&dec_number_add(&self.0, &rhs.0, &mut ctx), &mut ctx))
  }
}

impl std::ops::AddAssign<DecimalNumber> for DecimalNumber {
  ///
  fn add_assign(&mut self, rhs: Self) {
    let mut ctx = Self::default_context();
    self.0 = dec_number_reduce(&dec_number_add(&self.0, &rhs.0, &mut ctx), &mut ctx);
  }
}

impl std::ops::Sub<DecimalNumber> for DecimalNumber {
  type Output = Self;
  ///
  fn sub(self, rhs: Self) -> Self::Output {
    let mut ctx = Self::default_context();
    Self(dec_number_reduce(
      &dec_number_subtract(&self.0, &rhs.0, &mut ctx),
      &mut ctx,
    ))
  }
}

impl std::ops::SubAssign<DecimalNumber> for DecimalNumber {
  ///
  fn sub_assign(&mut self, rhs: Self) {
    let mut ctx = Self::default_context();
    self.0 = dec_number_reduce(&dec_number_subtract(&self.0, &rhs.0, &mut ctx), &mut ctx);
  }
}

impl std::ops::Mul<DecimalNumber> for DecimalNumber {
  type Output = Self;
  ///
  fn mul(self, rhs: Self) -> Self::Output {
    let mut ctx = Self::default_context();
    Self(dec_number_reduce(
      &dec_number_multiply(&self.0, &rhs.0, &mut ctx),
      &mut ctx,
    ))
  }
}

impl std::ops::MulAssign<DecimalNumber> for DecimalNumber {
  ///
  fn mul_assign(&mut self, rhs: Self) {
    let mut ctx = Self::default_context();
    self.0 = dec_number_reduce(&dec_number_multiply(&self.0, &rhs.0, &mut ctx), &mut ctx);
  }
}

impl std::ops::Div<DecimalNumber> for DecimalNumber {
  type Output = Self;
  ///
  fn div(self, rhs: Self) -> Self::Output {
    let mut ctx = Self::default_context();
    Self(dec_number_reduce(
      &dec_number_divide(&self.0, &rhs.0, &mut ctx),
      &mut ctx,
    ))
  }
}

impl std::ops::DivAssign<DecimalNumber> for DecimalNumber {
  ///
  fn div_assign(&mut self, rhs: Self) {
    let mut ctx = Self::default_context();
    self.0 = dec_number_reduce(&dec_number_divide(&self.0, &rhs.0, &mut ctx), &mut ctx);
  }
}

impl PartialEq<DecimalNumber> for DecimalNumber {
  ///
  fn eq(&self, rhs: &Self) -> bool {
    let mut ctx = Self::default_context();
    dec_number_is_zero(&dec_number_compare(&self.0, &rhs.0, &mut ctx))
  }
}

impl Eq for DecimalNumber {}

impl PartialOrd<DecimalNumber> for DecimalNumber {
  ///
  fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
    let mut ctx = Self::default_context();
    let flag = dec_number_compare(&self.0, &rhs.0, &mut ctx);
    if dec_number_is_zero(&flag) {
      return Some(Ordering::Equal);
    }
    if dec_number_is_negative(&flag) {
      return Some(Ordering::Less);
    }
    Some(Ordering::Greater)
  }
}

impl Ord for DecimalNumber {
  fn cmp(&self, rhs: &Self) -> Ordering {
    let mut ctx = Self::default_context();
    let flag = dec_number_compare(&self.0, &rhs.0, &mut ctx);
    if dec_number_is_zero(&flag) {
      return Ordering::Equal;
    }
    if dec_number_is_negative(&flag) {
      return Ordering::Less;
    }
    Ordering::Greater
  }
}

impl From<&str> for DecimalNumber {
  ///
  fn from(s: &str) -> Self {
    let mut ctx = Self::default_context();
    let dn = dec_number_from_string(s, &mut ctx);
    Self(dn)
  }
}

impl FromStr for DecimalNumber {
  type Err = Infallible;
  ///
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(s.into())
  }
}

impl From<u8> for DecimalNumber {
  /// Converts [DecimalNumber] from [u8].
  fn from(n: u8) -> Self {
    Self(dec_number_from_u32(n as u32))
  }
}

impl From<i8> for DecimalNumber {
  /// Converts [DecimalNumber] from [i8].
  fn from(n: i8) -> Self {
    Self(dec_number_from_i32(n as i32))
  }
}

impl From<u16> for DecimalNumber {
  /// Converts [DecimalNumber] from [u16].
  fn from(n: u16) -> Self {
    Self(dec_number_from_u32(n as u32))
  }
}

impl From<i16> for DecimalNumber {
  /// Converts [DecimalNumber] from [i16].
  fn from(n: i16) -> Self {
    Self(dec_number_from_i32(n as i32))
  }
}

impl From<u32> for DecimalNumber {
  /// Converts [DecimalNumber] from [u32].
  fn from(n: u32) -> Self {
    Self(dec_number_from_u32(n))
  }
}

impl From<i32> for DecimalNumber {
  /// Converts [DecimalNumber] from [i32].
  fn from(n: i32) -> Self {
    Self(dec_number_from_i32(n))
  }
}

impl From<u64> for DecimalNumber {
  /// Converts [DecimalNumber] from [u64].
  fn from(n: u64) -> Self {
    Self(dec_number_from_u64(n))
  }
}

impl From<i64> for DecimalNumber {
  /// Converts [DecimalNumber] from [i64].
  fn from(n: i64) -> Self {
    let mut ctx = Self::default_context();
    Self(dec_number_from_i64(n, &mut ctx))
  }
}

impl From<u128> for DecimalNumber {
  /// Converts [DecimalNumber] from [u128].
  fn from(n: u128) -> Self {
    Self(dec_number_from_u128(n))
  }
}

impl From<i128> for DecimalNumber {
  /// Converts [DecimalNumber] from [i128].
  fn from(n: i128) -> Self {
    let mut ctx = Self::default_context();
    Self(dec_number_from_i128(n, &mut ctx))
  }
}

impl From<usize> for DecimalNumber {
  /// Converts [DecimalNumber] from [usize].
  fn from(n: usize) -> Self {
    Self(dec_number_from_usize(n))
  }
}

impl From<isize> for DecimalNumber {
  /// Converts [DecimalNumber] from [isize].
  fn from(n: isize) -> Self {
    let mut ctx = Self::default_context();
    Self(dec_number_from_isize(n, &mut ctx))
  }
}
