use crate::dec_context::*;
use crate::dec_number::*;
use crate::ContextKind::Decimal128;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DecimalNumber(DecNumber);

impl Default for DecimalNumber {
  /// The default value of [DecimalNumber] is zero.
  fn default() -> Self {
    Self(DecNumber::default())
  }
}

impl std::fmt::Display for DecimalNumber {
  /// Converts [DecimalNumber] into string.
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", dec_number_to_string(&self.0))
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

impl DecimalNumber {
  ///
  fn default_context() -> DecContext {
    dec_context_default(Decimal128)
  }
  ///
  pub fn zero() -> Self {
    Self(DecNumber::zero())
  }
  ///
  pub fn one() -> Self {
    Self(DecNumber::one())
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
