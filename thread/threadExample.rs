use std::thread;
use std::time::Duration;

fn main() {
   
    thread::spawn(|| {
        for i in 0..10 {
            println!("this is from spawned thread:{}", i);
            
            if i == 0 {
                thread::sleep(Duration::from_secs(1));
            }
        }
    });
    thread::spawn(|| {
        for i in 0..10 {
            println!("this is from 2nd spawned thread:{}", i);
            //thread::sleep(Duration::from_secs(1));
        }
    });
    for i in 0..10 {
        println!("this is from main:{}", i);
        thread::sleep(Duration::from_secs(1));
    }
}
