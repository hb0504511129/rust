// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// compile-flags: -Z print-type-sizes

// This file illustrates how packing is handled; it should cause
// the elimination of padding that would normally be introduced
// to satisfy alignment desirata.

#![feature(untagged_unions)]

#![allow(dead_code)]

#[derive(Default)]
#[repr(packed)]
struct Packed {
    a: bool,
    b: bool,
    g: i32,
    c: bool,
    w: i64,
    d: bool,
}

#[derive(Default)]
struct Padded {
    a: bool,
    b: bool,
    g: i32,
    c: bool,
    w: i64,
    d: bool,
}

pub fn main() {
    let _c: Packed = Default::default();
    let _d: Padded = Default::default();
}
