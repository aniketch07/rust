use std::thread;
use std::sync::Arc;
use std::sync::mpsc;

fn main() {
    let (sender, receiver) =mpsc::channel();
    let shared_receiver = Arc::new(receiver);
    let clonerecv=shared_receiver.clone();
    let thread1=thread::spawn(move || {
        let x=Arc::new(12);
        println!("{}", x);
        sender.send(x);
    });
    //let recriver2=receiver.clone(); cannot use this bcs receiver does not implemmt clone method instead clone receiver 
    //using arc 
    let thread2=thread::spawn(move || {
        let y=shared_receiver.recv().unwrap().clone();
        println!("{}", y);
        // sender.send(x);
    });
    let result = clonerecv.recv().unwrap();
    println!("Hello spawned thread is asleep this is the main thread");
    println!("Result: {}", result);

    thread2.join().unwrap();
    thread1.join().unwrap();


}

//this goves error bcs Receiver does not implement clone method
// use std::thread;
// use std::sync::mpsc;
// use std::time::Duration;

// fn main() {
//     let (sender, receiver) = mpsc::channel();

//     let cloned_sender = sender.clone();
//     let task_executor = thread::spawn(move || {
//         println!("Hello this is spawned thread");
//         thread::sleep(Duration::from_secs(2));
//         let result = 40;
//         cloned_sender.send(result).unwrap();
//     });

//     let cloned_receiver = receiver.clone();
//     let time_worker = thread::spawn(move || {
//         let result_from_task_executor = cloned_receiver.recv().unwrap();
//         println!("this data is received from taskExecutor: {}", result_from_task_executor);
//         sender.send(result_from_task_executor).unwrap();
//     });

//     let result = receiver.recv().unwrap();
//     println!("Hello spawned thread is asleep this is the main thread");
//     println!("Result: {}", result);

//     time_worker.join().unwrap();
//     task_executor.join().unwrap();
// }

