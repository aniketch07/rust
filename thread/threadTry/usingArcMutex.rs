use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello world");
    let (sender, receiver) = mpsc::channel();
    let arc_receiver = Arc::new(Mutex::new(receiver));

    let task_executor = thread::spawn(move || {
        println!("Hello this is spawned thread");
        thread::sleep(Duration::from_secs(2));
        let result = 40;
        sender.send(result).unwrap();
    });

    let cloned_arc_receiver = Arc::clone(&arc_receiver);
    let time_worker = thread::spawn(move || {
        let result_from_task_executor = cloned_arc_receiver.lock().unwrap().recv().unwrap();
        println!("this data is received from taskExecutor: {}", result_from_task_executor);
        let cloned_sender = sender.clone();
        cloned_sender.send(result_from_task_executor).unwrap();
    });

    task_executor.join().unwrap();
    time_worker.join().unwrap();
    let result_from_time_worker = receiver.recv().unwrap();
    println!("this data is received from time_worker: {}", result_from_time_worker);
}
