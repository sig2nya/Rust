use std::thread;
use std::time::Duration;

fn main() {
    println!("Main Thread Started");
    let handle = thread::spawn(|| {
        println!("Thread Start Working");
        thread::sleep(Duration::from_secs(2));
        println!("Terminated Working");
    });

    println!("Main Thread : Working Others...");
    handle.join().unwrap();
    println!("Main Thread : Every Threads are Terminated and exit");
}
