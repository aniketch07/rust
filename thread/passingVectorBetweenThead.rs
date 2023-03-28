use std::thread;
use std::sync::mpsc;

fn main() {
    let (sender, receiver) =mpsc::channel();
   

    thread::spawn(move ||{
        let vec = vec![1,2,3,4];
        println!("this is thread 1");
        sender.send(vec);

    });
    let mut vec1=Vec::new();
    vec1=receiver.recv().unwrap();
    for i in 0..vec1.len() {
        println!("this is main thread{}",vec1[i]);
    }

}