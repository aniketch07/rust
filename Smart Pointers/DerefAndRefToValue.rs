fn main() {
    let x=12;

    // In Rust, the * symbol is used to dereference a pointer. 
//This means that if you have a pointer ptr that points 
// to a value in memory, you can use *ptr to access the actual value stored at that memory location.

    let ptrx=&x ;

    println!("{:p} {}",ptrx, *ptrx);



    //Sure! In Rust, the & symbol is used to create a reference to a value.
    // A reference is a way to allow a function or piece of code to borrow a value without taking ownership of it.

    let mut y=14;
    y=15;
    let a=&y;
    println!("{} {}",y, a);
}

// In Rust, the * symbol is used to dereference a pointer. 
//This means that if you have a pointer ptr that points 
// to a value in memory, you can use *ptr to access the actual value stored at that memory location.