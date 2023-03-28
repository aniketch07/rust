use std::thread;
use std::sync::mpsc;

fn main() {
    let (sender, receiver) =mpsc::channel();
    let thread = thread::spawn( ||{
        let data =12;
        println!("Send data: {} to main thread using channel", data);
        if let Err(e) = sender.send(data) {
            println!("Error sending data from thread1: {:?}", e);
        }

    });

    // if i make one more thread and send data to it and then after that send data to main thread
    //uisng a single channel it doesn't work properly
    let thread2 = thread::spawn( ||{
        let data =receiver.recv().unwrap();
        println!("received data {}", data);
        // if let Err(e) = sender.send(data) {
        //     println!("Error sending data from thread1: {:?}", e);
        // }

    });

    let received_data = receiver.recv();
    if let Err(e) = received_data {
        println!("Error receiving data in the main thread: {:?}", e);
    } else if let Ok(data) = received_data {
        println!("Hello, this is the main thread. I received some data from thread1: {}", data);
    }
    thread.join().unwrap();
    thread2.join().unwrap();
}