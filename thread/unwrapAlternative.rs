use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    let (sender, receiver) =mpsc::channel();

    let thread1=thread::spawn(move || {
        let data=12;
        if let Err(e) = sender.send(data) {
            println!("there was an error");

        }
    });
    thread1.join().unwrap();
    let received_data = receiver.recv();
    // if let Err(e) = received_data {
    //     println!("Error receiving data in the main thread: {:?}", e);
    // } else {
    //     println!("Hello, this is the main thread. I received some data from thread1: {:?}", received_data);
    // }
    if let Ok(data) = received_data {
        println!("{}", data);
    }
    else{
        println!("Error receiving data from thread");
    }
    

}

// use std::thread;
// use std::time::Duration;
// use std::sync::mpsc;

// fn main() {
//     let (sender, receiver) = mpsc::channel();

//     let thread1 = thread::spawn(move || {
//         let data = 12;
//         if let Err(e) = sender.send(data) {
//             println!("there was an error");
//         }
//     });

//     thread1.join().unwrap();

//     match receiver.recv_timeout(Duration::from_secs(1)) {
//         Ok(received_data) => println!("{}", received_data),
//         Err(_) => println!("No data received within timeout period"),
//     }
// }
