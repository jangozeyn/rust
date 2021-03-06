// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

enum Ast<'a> {
    Num(usize),
    Add(&'a Ast<'a>, &'a Ast<'a>)
}

fn build() {
    let x = Ast::Num(3);
    let y = Ast::Num(4);
    let z = Ast::Add(&x, &y);
    compute(&z);
}

fn compute(x: &Ast) -> usize {
    match *x {
      Ast::Num(x) => { x }
      Ast::Add(x, y) => { compute(x) + compute(y) }
    }
}

fn map_nums<'a,'b, F>(x: &Ast, f: &mut F) -> &'a Ast<'b> where F: FnMut(usize) -> usize {
    match *x {
      Ast::Num(x) => {
        return &Ast::Num((*f)(x)); //~ ERROR borrowed value does not live long enough
      }
      Ast::Add(x, y) => {
        let m_x = map_nums(x, f);
        let m_y = map_nums(y, f);
        return &Ast::Add(m_x, m_y);  //~ ERROR borrowed value does not live long enough
      }
    }
}

fn main() {}
