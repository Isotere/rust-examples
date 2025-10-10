use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;

static NUMBERS: Mutex<Vec<u32>> = Mutex::new(Vec::new());

fn main() {
    let mut handlers = Vec::new();
    for i in 0..10 {
        let handler = std::thread::spawn(move || {
            sleep(Duration::from_millis(rand::random::<u64>() % 500));
            let mut lock = NUMBERS.lock().unwrap();
            lock.push(i);
        });
        handlers.push(handler);
    }

    handlers.into_iter().for_each(|h| h.join().unwrap());

    let lock = NUMBERS.lock().unwrap();
    println!("{:#?}", lock);
}
