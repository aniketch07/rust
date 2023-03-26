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

    let (sender2, receiver2) = mpsc::channel();
    let timeWorker = thread::spawn(move || {
        let resultFromTaskExecutor = receiver.recv().unwrap();
        println!("this data is received from taskExecutor: {}", resultFromTaskExecutor);
        sender2.send(resultFromTaskExecutor).unwrap();
    });

    let result = receiver2.recv().unwrap();
    println!("Hello spawned thread is asleep this is the main thread");
    println!("Result: {}", result);

    taskExecutor.join().unwrap();
    timeWorker.join().unwrap();
}
