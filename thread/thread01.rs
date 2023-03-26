use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    let (sender, receiver) =mpsc::channel();

    let thread1=thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        println!("Hello this is thread1 i am sending some data to main thread");
        let data=12;
        sender.send(data).unwrap();
    });
    thread1.join().unwrap();
    println!("Hello this is main thread i received some data from thread1:{}",receiver.recv().unwrap());
    
}