use std::sync::Arc;
use std::thread;

fn main() {
    let x = Arc::new(12);

    let thread1 = thread::spawn({
        let val = x.clone();
        move || {
            println!("{}", val);
        }
    });

    let thread2 = thread::spawn({
        let val1 = x.clone();
        move || {
            println!("{}", val1);
        }
    });

    let thread3 = thread::spawn({
        let val2 = x.clone();
        move || {
            println!("{}", val2);
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    thread3.join().unwrap();
}

// use std::sync::Arc;
// use std::thread;

// fn main() {

//     let x=Arc::new(12);
    

//     let thread1=thread::spawn(move || {
//         let val=x.clone();
//         println!("{}", val);
//     });
//     let thread2=thread::spawn(move || {
//         let val1=x.clone();
//         println!("{}", val1);
//     });
//     let thread3=thread::spawn(move || {
//         let val2=x.clone();
//         println!("{}", val2);
//     });

//     thread1.join().unwrap();
//     thread2.join().unwrap();
//     thread3.join().unwrap();
// }


// for i in 0..5 {
//     let value = x.clone();

//     thread::spawn(move || {
//         println!("Thread {} sees value {}", i, value);
//     });
// }