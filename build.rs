/*
 * MIT License
 *
 * Copyright (c) 2022 senees
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

fn main() {
  let output_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
  let target_endian = if cfg!(target_endian = "little") { "1" } else { "0" };
  cc::Build::new()
    .define("DECLITEND", Some(target_endian))
    .file("./decNumber-icu-368/decimal128.c")
    .file("./decNumber-icu-368/decimal64.c")
    .file("./decNumber-icu-368/decimal32.c")
    .file("./decNumber-icu-368/decContext.c")
    .file("./decNumber-icu-368/decNumber.c")
    .file("./decNumber-icu-368/decSingle.c")
    .file("./decNumber-icu-368/decDouble.c")
    .file("./decNumber-icu-368/decQuad.c")
    .file("./decNumber-icu-368/decPacked.c")
    .out_dir(output_dir.join("lib"))
    .compile("decnumber-icu-368");
}
