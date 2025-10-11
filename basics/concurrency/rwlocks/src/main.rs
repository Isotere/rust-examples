use std::sync::{LazyLock, RwLock};

static USERS: LazyLock<RwLock<Vec<String>>> = LazyLock::new(|| RwLock::new(build_users()));

fn build_users() -> Vec<String> {
    vec![
        "alice".to_string(),
        "bob".to_string(),
        "charlie".to_string(),
    ]
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

fn main() {
    std::thread::spawn(|| {
        loop {
            println!("Current users (in thread):");
            let users = USERS.read().unwrap();
            println!("{users:?}");

            std::thread::sleep(std::time::Duration::from_secs(3));
        }
    });

    loop {
        println!("Print username to add to users list: ");
        let input = read_line();

        if input == "q" {
            break;
        }

        let mut users = USERS.write().unwrap();
        users.push(input);
    }
}
