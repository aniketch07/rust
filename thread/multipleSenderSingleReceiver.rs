// use std::thread;
// use std::sync::mpsc;

// fn main(){
//     let (sender,receiver)=mpsc::channel();
//     let thread1=thread::spawn(move ||{
//         let data1=12;
//         sender.send(data1);

//     });
//     let thread2=thread::spawn(move ||{
//         let data2=13;
//         sender.send(data2);

//     });
    
//     let thread3=thread::spawn(move ||{
//         let data3=14;
//         sender.send(data3);

//     });
//     let received=thread::spawn(move ||{
        
//         let data=receiver.recv();
//         println!("Received data: {:?}", data);

//     });

//     thread1.join().unwrap();
//     thread2.join().unwrap();
//     thread3.join().unwrap();
//     received.join().unwrap();
    


// }

//sending data from multiple thread to a single thread using cloned thread
use std::thread;
use std::sync::mpsc;

fn main() {
    let (sender, receiver) = mpsc::channel();

    // Spawn three threads that send data to the main thread
    let thread1 = {
        let sender = sender.clone();
        thread::spawn(move || {
            let data=1;
            sender.send(data).unwrap();
        })
    };
    let thread2 = {
        let sender = sender.clone();
        thread::spawn(move || {
            let data=2;
            sender.send(data).unwrap();
        })
    };
    let thread3 = {
        let sender = sender.clone();
        thread::spawn(move || {
            let data=3;
            sender.send(data).unwrap();
        })
    };

    // make a new channel to send data from this thrad to main thread 
    let (sender2, receiver2) = mpsc::channel();

    let receiverThread = thread::spawn(move || {
       // let sender = sender.clone();
        
        for _ in 0..3 {
            let data = receiver.recv().unwrap();
            println!("{}", data);
            sender2.send(data).unwrap();
        }
        
    });

    // Receive data from all three threads
   // _ is used as a placeholder for unused value from the loop
    for _ in 0..3 {
        println!("this is in main thread:{}", receiver2.recv().unwrap());
    }

    // Wait for all threads to finish
    thread1.join().unwrap();
    thread2.join().unwrap();
    thread3.join().unwrap();
    receiverThread.join().unwrap();
}


 
// thread ->thread2->main 