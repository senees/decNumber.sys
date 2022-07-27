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
use crate::dec_quad::*;
use crate::DecContext;

#[rustfmt::skip]
extern "C" {
  ///
  pub fn decimal128ToNumber(arg1: *const DecQuad, arg2: *mut DecNumber) -> *mut DecNumber;

  ///
  pub fn decimal128FromNumber(arg1: *mut DecQuad, arg2: *const DecNumber, arg3: *mut DecContext) -> *mut DecQuad;
}
