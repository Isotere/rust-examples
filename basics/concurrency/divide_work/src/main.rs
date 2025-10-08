const N_THREADS: usize = 8;

fn main() {
    let to_add: Vec<u32> = (0..5000).collect();
    let mut handlers = Vec::new();
    let chunks = to_add.chunks(N_THREADS);

    for chunk in chunks {
        let my_chunk = chunk.to_owned();
        handlers.push(std::thread::spawn(move || my_chunk.iter().sum::<u32>()));
    }

    let mut sum = 0;
    for handler in handlers {
        sum += handler.join().unwrap();
    }

    println!("Sum: {}", sum);
}
