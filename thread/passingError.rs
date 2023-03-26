use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    let (sender, receiver) = mpsc::channel::<Result<i32, String>>();

    let thread1 = thread::spawn(move || {
        println!("Hello, this is thread1. I am sending an error to the main thread.");
        let err = "Some error occurred".to_string();
        let data:i32=12;
        let send_result = sender.send(Ok(data));
        if let Err(e) = send_result {
            println!("Error sending error from thread1: {:?}", e);
        }
    });

    thread1.join().unwrap();

    let received_result = receiver.recv();
    if let Err(e) = received_result {
        println!("Error receiving error in the main thread: {:?}", e);
    } else if let Ok(result) = received_result {
        println!("Hello, this is the main thread. I received data from thread1 {:?}", result.to_owned());
    }
}
