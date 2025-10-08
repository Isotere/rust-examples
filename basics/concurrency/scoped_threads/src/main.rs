const N_THREADS: usize = 8;

fn main() {
    let to_add = (0..5000).collect::<Vec<u32>>();
    let chunks = to_add.chunks(N_THREADS);

    let sum = std::thread::scope(|s| {
        let mut handlers = Vec::new();

        for chunk in chunks {
            let handler = s.spawn(move || chunk.iter().sum::<u32>());
            handlers.push(handler);
        }

        handlers.into_iter().map(|h| h.join().unwrap()).sum::<u32>()
    });

    println!("Sum: {:?}", sum);
}
