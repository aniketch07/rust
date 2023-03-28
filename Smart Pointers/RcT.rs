use std::rc::Rc;

fn main() {
    let x=Rc::new(12);

    let y=Rc::clone(&x);
    let z=Rc::clone(&x);
    println!("{:?}", x);
    println!("{:?}", y);
    println!("{:?}", z);
    println!("{:?}", Rc::strong_count(&x));


}