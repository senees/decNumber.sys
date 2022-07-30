//! Declarations of external functions linked from `The decNumber C Library`.
//!
//! For copyright and licensing info see the note below
//! and the content of the **ICU-license.html** file.
//!
//! ```text
//! /// ------------------------------------------------------------------ */
//! /// Copyright (c) IBM Corporation, 2000, 2006.  All rights reserved.   */
//! ///                                                                    */
//! /// This software is made available under the terms of the             */
//! /// ICU License -- ICU 1.8.1 and later.                                */
//! ///                                                                    */
//! /// The description and User's Guide ("The decNumber C Library") for   */
//! /// this software is called decNumber.pdf.  This document is           */
//! /// available, together with arithmetic and format specifications,     */
//! /// testcases, and Web links, on the General Decimal Arithmetic page.  */
//! ///                                                                    */
//! /// Please send comments, suggestions, and corrections to the author:  */
//! ///   mfc@uk.ibm.com                                                   */
//! ///   Mike Cowlishaw, IBM Fellow                                       */
//! ///   IBM UK, PO Box 31, Birmingham Road, Warwick CV34 5JL, UK         */
//! /// ------------------------------------------------------------------ */
//! ```

use crate::dec_number::DecNumber;
use crate::DecContext;
use libc::{c_char, c_int, c_uint};

#[rustfmt::skip]
extern "C" {
  pub fn decNumberAdd(res: *mut DecNumber, dn1: *const DecNumber, dn2: *const DecNumber, ctx: *mut DecContext) -> *mut DecNumber;
  pub fn decNumberDivide(res: *mut DecNumber, dn1: *const DecNumber, dn2: *const DecNumber, ctx: *mut DecContext) -> *mut DecNumber;
  pub fn decNumberCompare(res: *mut DecNumber, dn1: *const DecNumber, dn2: *const DecNumber, ctx: *mut DecContext) -> *mut DecNumber;
  pub fn decNumberExp(res: *mut DecNumber, dn: *const DecNumber, ctx: *mut DecContext) -> *mut DecNumber;
  pub fn decNumberFromInt32(res: *mut DecNumber, n: c_int) -> *mut DecNumber;
  pub fn decNumberFromString(res: *mut DecNumber, s: *const c_char, ctx: *mut DecContext) -> *mut DecNumber;
  pub fn decNumberFromUInt32(res: *mut DecNumber, n: c_uint) -> *mut DecNumber;
  pub fn decNumberLn(res: *mut DecNumber, dn: *const DecNumber, ctx: *mut DecContext) -> *mut DecNumber;
  pub fn decNumberMinus(res: *mut DecNumber, dn: *const DecNumber, ctx: *mut DecContext) -> *mut DecNumber;
  pub fn decNumberMultiply(res: *mut DecNumber, dn1: *const DecNumber, dn2: *const DecNumber, ctx: *mut DecContext) -> *mut DecNumber;
  pub fn decNumberReduce(res: *mut DecNumber, dn: *const DecNumber, ctx: *mut DecContext) -> *mut DecNumber;
  pub fn decNumberRescale(res: *mut DecNumber, lhs: *const DecNumber, rhs: *const DecNumber, ctx: *mut DecContext) -> *mut DecNumber;
  pub fn decNumberSubtract(res: *mut DecNumber, dn1: *const DecNumber, dn2: *const DecNumber, ctx: *mut DecContext) -> *mut DecNumber;
  pub fn decNumberToString(dn: *const DecNumber, s: *mut c_char) -> *mut c_char;
  pub fn decNumberZero(res: *mut DecNumber);
}
