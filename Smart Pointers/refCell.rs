use std::cell::RefCell;
use std::fmt::Display;
fn main() {
    let x=RefCell::new(vec![1,2,3]);

    let mut y=x.borrow_mut();
    y.push(1);
    for i in 0..y.len() {
        println!("Value: {}", y[i]);
    }
   
}