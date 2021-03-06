// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(const_let)]

pub static mut A: u32 = 0;
pub static mut B: () = unsafe { A = 1; };
//~^ ERROR could not evaluate static initializer

pub static mut C: u32 = unsafe { C = 1; 0 };
//~^ ERROR cycle detected

pub static D: u32 = D;

fn main() {}
