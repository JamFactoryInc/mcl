extern crate derive_more;

mod vm;
mod translate;
mod src_in;
mod parse;
mod grammar;
mod util;

fn main() {
    println!("Hello, world!");

    let mut x = vec![1, 2, 3];
    let mut y = x.clone();
    y[1] = 4;
    println!("{}", x[1])

}