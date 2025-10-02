use console::read_line;

fn main() {
    match read_line() {
        Ok(line) => {
            println!("Input is: [{}]", line);
        }
        Err(error) => {
            eprintln!("{:?}", error);
        }
    }
}
