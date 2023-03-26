use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    let (sender, receiver) = mpsc::channel();

    let thread1 = thread::spawn(move || {
        println!("Hello, this is thread1. I am sending some data to the main thread.");
        let data = 12;
        let send_result = sender.send(data);
        if let Err(e) = send_result {
            println!("Error sending data from thread1: {:?}", e);
        }
    });

    thread1.join().unwrap();

    let received_data = receiver.recv();
    if let Err(e) = received_data {
        println!("Error receiving data in the main thread: {:?}", e);
    } else if let Ok(data) = received_data {
        println!("Hello, this is the main thread. I received some data from thread1: {}", data);
    }
}
