static mut COUNTER: u32 = 0;

fn main() {
    let mut handlers = Vec::new();
    for _ in 0..1000 {
        let handle = std::thread::spawn(|| {
            for _ in 0..1_100 {
                unsafe {
                    COUNTER += 1;
                }
            }
        });
        handlers.push(handle);
    }

    handlers.into_iter().for_each(|h| h.join().unwrap());

    println!("Counter: {:?}", unsafe { COUNTER });
}
