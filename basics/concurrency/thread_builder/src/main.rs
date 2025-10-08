fn my_thread() {
    println!(
        "Hello from thread named as: {}",
        std::thread::current().name().unwrap()
    );
}

fn main() {
    let handler = std::thread::Builder::new()
        // Проще для дебага или логов
        .name(String::from("Named thread"))
        // ускоряем запуск
        .stack_size(std::mem::size_of::<usize>() * 4)
        .spawn(my_thread)
        .unwrap();

    handler.join().unwrap();
}
