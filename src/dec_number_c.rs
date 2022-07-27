//! Declarations of external functions linked from `The decNumber C Library`.
//!
//! Most of the function descriptions are taken from
//! the source code of this library.
//!
//! For copyright and licensing info see the note below
//! and the content of the **ICU-license.html** file.
//!
//! ```text
//! /* ------------------------------------------------------------------ */
//! /* Copyright (c) IBM Corporation, 2000, 2006.  All rights reserved.   */
//! /*                                                                    */
//! /* This software is made available under the terms of the             */
//! /* ICU License -- ICU 1.8.1 and later.                                */
//! /*                                                                    */
//! /* The description and User's Guide ("The decNumber C Library") for   */
//! /* this software is called decNumber.pdf.  This document is           */
//! /* available, together with arithmetic and format specifications,     */
//! /* testcases, and Web links, on the General Decimal Arithmetic page.  */
//! /*                                                                    */
//! /* Please send comments, suggestions, and corrections to the author:  */
//! /*   mfc@uk.ibm.com                                                   */
//! /*   Mike Cowlishaw, IBM Fellow                                       */
//! /*   IBM UK, PO Box 31, Birmingham Road, Warwick CV34 5JL, UK         */
//! /* ------------------------------------------------------------------ */
//! ```

use crate::bindings::*;
use libc::{c_char, c_int, c_uint};

#[rustfmt::skip]
extern "C" {
  ///
  pub fn decimal128ToNumber(arg1: *const DecQuad, arg2: *mut DecNumber) -> *mut DecNumber;
  ///
  pub fn decimal128FromNumber(arg1: *mut DecQuad, arg2: *const DecNumber, arg3: *mut DecContext) -> *mut DecQuad;

  pub fn decNumberRescale(arg1: *mut DecNumber, arg2: *const DecNumber, arg3: *const DecNumber, arg4: *mut DecContext) -> *mut DecNumber;
  
  /// `decContextDefault` initializes a context structure.
  ///
  /// `context` is the structure to be initialized.
  ///
  /// `kind` selects the required set of default values, one of:
  ///  - `DEC_INIT_BASE`       selects ANSI X3-274 defaults,
  ///  - `DEC_INIT_DECIMAL32`  selects IEEE 754 defaults, 32-bit,
  ///  - `DEC_INIT_DECIMAL64`  selects IEEE 754 defaults, 64-bit,
  ///  - `DEC_INIT_DECIMAL128` selects IEEE 754 defaults, 128-bit,
  ///  - for any other value a valid context is returned, but with
  ///    `Invalid_operation` set in the status field.
  ///
  ///  Returns a `context` structure with the appropriate initial values.
  ///
  pub fn decContextDefault(context: *mut DecContext, kind: i32) -> *mut DecContext;
  /// `decQuadAdd` adds two decQuads.
  ///
  /// `result` gets the result of adding `dql` and `dqr`.
  /// `dql`    is the first decQuad (lhs).
  /// `dqr`    is the second decQuad (rhs).
  /// `set`    is the context.
  /// 
  ///  Returns `result`.
  ///
  pub fn decQuadAdd(result: *mut DecQuad, dql: *const DecQuad, dqr: *const DecQuad, set: *mut DecContext) -> *mut DecQuad;
  ///
  pub fn decQuadFromInt32(arg1: *mut DecQuad, arg2: c_int) -> *mut DecQuad;
  ///
  pub fn decQuadFromUInt32(arg1: *mut DecQuad, arg2: c_uint) -> *mut DecQuad;
  /* ------------------------------------------------------------------ */
  /* decFloatFromString -- conversion from numeric string               */
  /*                                                                    */
  /*  result  is the decFloat format number which gets the result of    */
  /*          the conversion                                            */
  /*  *string is the character string which should contain a valid      */
  /*          number (which may be a special value), \0-terminated      */
  /*          If there are too many significant digits in the           */
  /*          coefficient it will be rounded.                           */
  /*  set     is the context                                            */
  /*  returns result                                                    */
  pub fn decQuadFromString(result: *mut DecQuad, string: *const c_char, set: *mut DecContext) -> *mut DecQuad;
  ///
  pub fn decQuadToString(arg1: *const DecQuad, arg2: *mut c_char) -> *mut c_char;
}
