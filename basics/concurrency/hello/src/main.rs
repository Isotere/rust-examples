fn hello_thread(n: i32) {
    println!("Hello from thread [{}]!", n);
}

fn do_math(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("Hello from main!");

    let mut thread_pool = Vec::new();

    for i in 0..10 {
        // Так как i существует только в течение цикла, мы должны передать владение в замыкание. В данном случае
        // так как copy-тип - то будет скопировано в замыкание значение
        thread_pool.push(std::thread::spawn(move || do_math(5, i)));
    }

    println!("Hello from main again!");

    thread_pool.into_iter().for_each(|t| {
        println!("Math is: {}", t.join().unwrap());
    });
}
