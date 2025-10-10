use std::sync::atomic::AtomicU32;

static COUNTER: AtomicU32 = AtomicU32::new(0);

fn main() {
    let mut handlers = Vec::new();
    for _ in 0..1000 {
        let handle = std::thread::spawn(|| {
            for _ in 0..1_100 {
                COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
        });
        handlers.push(handle);
    }

    handlers.into_iter().for_each(|h| h.join().unwrap());

    println!(
        "Counter: {:?}",
        COUNTER.load(std::sync::atomic::Ordering::Relaxed)
    );
}
