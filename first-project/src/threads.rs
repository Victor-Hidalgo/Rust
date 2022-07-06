use std::thread;
use std::time::Duration;

pub fn run(){

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Number {} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("Number {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
}