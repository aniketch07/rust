// use std::thread;

// const NTHREADS: u32 = 10;


// fn main() {
//     let mut data = 10;
    
//     let thread=thread::spawn(move || {
//         data=data+1;
//         println!("data value in thread 1: {:?}",data);

//         println!("constant value: {:?}",NTHREADS);
//     });
//     let _=thread.join().unwrap();
//     println!("data value: {:?}",data);

//     println!("constant value: {:?}",NTHREADS);
// }
use std::thread;

const NTHREADS: u32 = 10;

fn main() {
    let mut data = 10;

    let thread = thread::spawn(move || {
        data = data + 1;
        println!("data value in thread: {:?}", data);
        println!("constant value: {:?}", NTHREADS);
    });

    thread.join().unwrap();

    println!("data value in main thread: {:?}", data);
    println!("constant value: {:?}", NTHREADS);
}
