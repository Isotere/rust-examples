fn test(n: u32) {
    println!("Test {n}");
}

fn main() {
    // Explicitly sized pool
    let pool = rayon::ThreadPoolBuilder::new().num_threads(4).build().unwrap();

    pool.spawn(|| println!("Hello from pool thread!"));

    pool.scope(|s| {
        for n in 0..20 {
            s.spawn(move |_| println!("Hello from scope thread {}", n));
        }
    });

    println!("Hello from main thread!");

    println!("\nNow check broadcast!");
    pool.scope(|s| {
        s.spawn_broadcast(|_s, ctx| {
            println!("Hello from broadcast thread {}!", ctx.index());
        })
    });

    println!("\nNow test Test and Join");

    pool.join(|| test(1), || test(2));
}
