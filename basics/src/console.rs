use crate::errors::MyCustomError;

pub fn read_line() -> Result<String, MyCustomError> {
    let mut input = String::new();

    match std::io::stdin().read_line(&mut input) {
        Ok(_) => Ok(input),
        Err(error) => Err(MyCustomError::ConsoleReadError(error.to_string())),
    }
}
