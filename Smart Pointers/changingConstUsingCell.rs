//can  we can change constant value using cell in rust????
// no we cannot change constant value in rust uisng cell or refCell

// but it doesn't give an error so idk why it is allowed

// Cell<T> is useful for providing interior mutability for values that are not defined as constants,
// but it cannot be used to change the value of a constant in Rust.

use std::cell::Cell;

const X: Cell<i32> = Cell::new(42);

fn main() {
    println!("{}", X.get()); // prints "42"
    X.set(43);
    println!("{}", X.get()); // prints "43" no it doesnt print 43 changing value of constant is not allowed
}
