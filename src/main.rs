#![feature(core_intrinsics)]
#![feature(generic_const_exprs)]
#![feature(portable_simd)]
extern crate derive_more;
extern crate enum_parser;

mod grammar;
mod parse;
mod src_in;
mod translate;
mod util;
mod vm;

fn main() {
    enum_parser::enum_parser!(
        SomeName, "one", "only", "onus", "ocre", "1two", "++", "-=", "&&", "==", "||", "@p"
    );

    println!("Hello, world!");

    let mut x = vec![1, 2, 3];
    let mut y = x.clone();
    y[1] = 4;
    println!("{}", x[1])
}
