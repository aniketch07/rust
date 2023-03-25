use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main(){

     let (sender, receiver) =mpsc::channel();
    let thread1=thread::spawn(move ||{
        println!("This is from thread one");
        sender.send(12);
    });

    let thread2=thread::spawn(move ||{
        for i in 0..10{
            println!("this data is from thread 2:{}", i);
            
        }
        thread::sleep(Duration::from_secs(1));
    });

    println!("This data is sent from thread one to main thread {}",receiver.recv().unwrap());
    thread2.join().unwrap();

}