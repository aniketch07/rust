use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::fmt::Display;

fn main() {
    let (sender,receiver)=mpsc::channel();
    // let thread1={
    //     let sender = sender.clone();
    //     thread::spawn(move || {

    //         let data1=12;
    //         sender.send(data1);
    //     });
    // };
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

    for i in 0..3{
        println!("{:?}",receiver.recv());
    }
    thread1.join().unwrap();
    thread2.join().unwrap();
    thread3.join().unwrap();
    
}