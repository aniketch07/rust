//this doesnt work because ownership is not allowed

use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (sender,receiver) = mpsc::channel();

    let taskExecutor = thread::spawn(move || {
        println!("Hello this is spawned thread");
        thread::sleep(Duration::from_secs(2));
        let result = 40;
        sender.send(result).unwrap();
    });

    let timeWorker = thread::spawn(move || {
        let resultFromTaskExecutor = receiver.recv().unwrap();
        println!("this data is received from taskExecutor: {}", resultFromTaskExecutor);
        sender.send(resultFromTaskExecutor).unwrap();
    });

    let result = receiver.recv().unwrap();
    println!("Hello spawned thread is asleep this is the main thread");
    println!("Result: {}", result);

    timeWorker.join().unwrap();
    taskExecutor.join().unwrap();
}
