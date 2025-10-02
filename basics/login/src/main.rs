use authentication::{login, LoginAction, LoginRole};
use console::read_line;

fn main() {
    let mut tries: i32 = 0;
    loop {
        println!("Enter your username: ");
        let username = read_line().unwrap();
        println!("Enter your password: ");
        let password = read_line().unwrap();

        match login(&username, &password) {
            Some(LoginAction::Granted(LoginRole::Admin)) => println!("You are logged in as admin!"),
            Some(LoginAction::Granted(LoginRole::User)) => println!("You are logged in as user!"),
            Some(LoginAction::Denied) => {
                println!("Invalid username or password");
                tries += 1;
                if tries < 3 {
                    continue;
                }

                println!("Too many tries");
            }
            None => {
                println!("User not found");
            }
        }
        break;
    }
}
