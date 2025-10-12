use std::sync::mpsc;
enum Command {
    SayHello,
    Quit,
}

fn main() {
    let (tx, rx) = mpsc::channel::<Command>();

    let handler = std::thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            match msg {
                Command::SayHello => println!("Hello from channel!"),
                Command::Quit => {
                    println!("Bye!");
                    break;
                }
            }
        }
    });

    for _ in 0..10 {
        tx.send(Command::SayHello).unwrap();
    }

    println!("Sending quit!");
    tx.send(Command::Quit).unwrap();

    handler.join().unwrap();
}
