use authentication::login;
use console::read_line;

fn main() {
    let mut tries: i32 = 0;
    loop {
        println!("Enter your username: ");
        let username = read_line().unwrap();
        println!("Enter your password: ");
        let password = read_line().unwrap();

        if login(&username, &password) {
            println!("Logged in successfully");
            break;
        }

        println!("Invalid username or password");
        tries += 1;
        if tries >= 3 {
            println!("Too many tries");
            break;
        }
    }
}
