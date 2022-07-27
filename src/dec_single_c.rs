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

use crate::dec_single::DecSingle;
use crate::DecContext;
use libc::c_char;

#[rustfmt::skip]
extern "C" {
  pub fn decSingleFromString(res: *mut DecSingle, s: *const c_char, ctx: *mut DecContext) -> *mut DecSingle;
  //pub fn decSingleFromWider(res: *mut DecSingle, ds: *const DecDouble, ctx: *mut DecContext) -> *mut DecSingle;
  pub fn decSingleToString(dq: *const DecSingle, s: *mut c_char) -> *mut c_char;
  //pub fn decSingleToWider(res: *const DecSingle, ds: *mut DecDouble, ctx: *mut DecContext) -> *mut DecDouble;
  pub fn decSingleZero(res: *mut DecSingle);
}
