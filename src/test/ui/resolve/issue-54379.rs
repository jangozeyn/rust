// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
struct MyStruct {
    pub s1: Option<String>,
}

fn main() {
    let thing = MyStruct { s1: None };

    match thing {
        MyStruct { .., Some(_) } => {}, //~ ERROR pattern does not mention field `s1`
        //~^ ERROR expected `,`
        //~| ERROR expected `}`, found `,`
        _ => {}
    }
}
