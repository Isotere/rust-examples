use crate::console::read_line;

mod console;
mod errors;

fn main() {
    match read_line() {
        Ok(line) => {
            println!("{}", line);
        }
        Err(error) => {
            eprintln!("{:?}", error);
        }
    }
}
