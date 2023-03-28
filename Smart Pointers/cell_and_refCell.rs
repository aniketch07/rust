use std::cell::*;

fn main() {
    let  num=Cell::new(10);

    println!("{}", num.get());

    num.set(1);
    println!("{}", num.get());
}