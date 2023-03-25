use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (sender,receiver) =mpsc::channel();

   let taskExectuer= thread::spawn(move || {
        println!("Hello this is spawned thread");
        thread::sleep(Duration::from_secs(2));
        let result = 40;
       
        
        sender.send(&result).unwrap();
       
    });
    
    let timeWorker= thread::spawn(move || {
        let resultFromtaskExecutor =receiver.recv().unwrap();
        println!("this data is received from taskExexutor{}", resultFromtaskExecutor);
        
        sender.send(&resultFromtaskExecutor).unwrap();
    });


    

    let result = receiver.recv().unwrap();
    println!("Hello sapwned thread is asleep this is the main thread");
    println!("Result: {}", result);
   

}