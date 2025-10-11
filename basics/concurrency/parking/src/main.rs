fn parkable_thread(n: i32) {
    loop {
        std::thread::park();
        println!("Thread {} is unparked", n);
    }
}

fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main() {
    let mut handlers = Vec::new();

    for i in 0..10 {
        handlers.push(std::thread::spawn(move || parkable_thread(i)))
    }

    loop {
        println!("Please enter thread [0-9] to unpark: ");
        let input = read_line();
        if input == "q" {
            break;
        }

        if let Ok(n) = input.parse::<usize>()
            && (0..10).contains(&n)
        {
            handlers[n].thread().unpark();
        }
    }
}
