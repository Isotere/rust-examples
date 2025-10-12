use std::sync::mpsc;

type Job = Box<dyn FnOnce() + Send + 'static>;

fn hi_there() {
    println!("hi there");
}

fn main() {
    let (tx, rx) = mpsc::channel::<Job>();

    let handler = std::thread::spawn(move || {
        while let Ok(job) = rx.recv() {
            job();
        }
    });

    let job = || println!("Hello from a closure!");
    let job_2 = || {
        for i in 0..10 {
            println!("hi there from main: {}", i);
        }
    };

    tx.send(Box::new(hi_there)).unwrap();
    tx.send(Box::new(job)).unwrap();
    tx.send(Box::new(job_2)).unwrap();

    handler.join().unwrap();
}
