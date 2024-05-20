use std::time::Duration;
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for i in 1..10 {
            println!("A pošiljam {}", i);
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    
});

 thread::spawn(move || {
        for i in 1..10 {
            println!("B pošiljam {}", i);
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    
});
    for i in 1..=5 {
        let v = rx.recv();
        println!("na {}-tem koraku sem prebral {}.", i, v);
        thread::sleep(Duration::from_millis(10));
    }
}
