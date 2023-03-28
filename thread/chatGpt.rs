// // //this also doesnt work 

// // use std::sync::{mpsc, Arc};
// // use std::thread;
// // use std::time::Duration;

// // fn main() {
// //     //let (sender, receiver) = mpsc::channel();
// //     let (sender,receiver) = Arc::new(mpsc::channel());
// //     let shared_sender = Arc::new(sender);
// //     let shared_receiver = Arc::new(receiver);

// //     let cloned_sender = shared_sender.clone();
// //     let taskExecutor = thread::spawn(move || {
// //         println!("Hello this is spawned thread");
// //         thread::sleep(Duration::from_secs(2));
// //         let result = 40;
// //         cloned_sender.send(result).unwrap();
// //     });

// //     let cloned_receiver = shared_receiver.clone();
// //     let cloned_sender = shared_sender.clone();
// //     let timeWorker = thread::spawn(move || {
// //         let resultFromTaskExecutor = cloned_receiver.recv().unwrap();
// //         println!("this data is received from taskExecutor: {}", resultFromTaskExecutor);
// //         cloned_sender.send(resultFromTaskExecutor).unwrap();
// //     });

// //     let result = shared_receiver.recv().unwrap();
// //     println!("Hello spawned thread is asleep this is the main thread");
// //     println!("Result: {}", result);

// //     timeWorker.join().unwrap();
// //     taskExecutor.join().unwrap();
// // }
// use std::thread;
// use std::sync::{mpsc, Arc};
// use std::time::Duration;

// fn main() {
//     let (sender, receiver) = mpsc::channel();
//     let sender = Arc::new(sender);
//     let receiver = Arc::new(receiver);

//     let cloned_sender = sender.clone();
//     let taskExecutor = thread::spawn(move || {
//         println!("Hello this is spawned thread");
//         thread::sleep(Duration::from_secs(2));
//         let result = 40;
//         cloned_sender.send(result).unwrap();
//     });

//     let cloned_receiver = receiver.clone();
//     let cloned_sender = sender.clone();
//     let timeWorker = thread::spawn(move || {
//         let resultFromTaskExecutor = cloned_receiver.recv().unwrap();
//         println!("this data is received from taskExecutor: {}", resultFromTaskExecutor);
//         cloned_sender.send(resultFromTaskExecutor).unwrap();
//     });

//     let result = receiver.recv().unwrap();
//     println!("Hello spawned thread is asleep this is the main thread");
//     println!("Result: {}", result);

//     timeWorker.join().unwrap();
//     taskExecutor.join().unwrap();
// }

use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::time::Duration;

fn main() {
    let (sender, receiver) = mpsc::channel();

    let cloned_sender = sender.clone();
    let task_executor = thread::spawn(move || {
        println!("Hello this is spawned thread");
        thread::sleep(Duration::from_secs(2));
        let result = 40;
        cloned_sender.send(result).unwrap();
    });

    let arc_receiver = Arc::new(receiver);
    let cloned_arc_receiver = arc_receiver.clone();
    let time_worker = thread::spawn(move || {
        let result_from_task_executor = cloned_arc_receiver.recv().unwrap();
        println!("this data is received from taskExecutor: {}", result_from_task_executor);
        let cloned_sender = sender.clone();
        cloned_sender.send(result_from_task_executor).unwrap();
    });

    let result = arc_receiver.recv().unwrap();
    println!("Hello spawned thread is asleep this is the main thread");
    println!("Result: {}", result);

    time_worker.join().unwrap();
    task_executor.join().unwrap();
}
