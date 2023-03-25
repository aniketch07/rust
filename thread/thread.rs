use std::thread;
use std::time::Duration;

fn main() {
    thread::sleep(Duration::from_secs(1));

    thread::spawn(move || {
        for i in 0..10 {
            println!("this is from spawned thread{}", i);
        }

    });
    //handle.join().unwrap();

   
    println!("this is from main thread");
}








//handle.join().unwrap(); 
    // thread::spawn(|| {
    //     for i in 0..10 {
    //         println!("this is from spawned thread:{}", i);
            
    //         if i == 0 {
    //             thread::sleep(Duration::from_secs(1));
    //         }
    //     }
    // });