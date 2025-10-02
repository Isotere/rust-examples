pub fn greet_user(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub fn login(username: &str, password: &str) -> bool {
    username.to_lowercase() == "admin" && password == "password"
}

#[cfg(test)]
mod tests {
    use crate::{greet_user, login};

    #[test]
    fn test_greet_user() {
        assert_eq!(greet_user("John Doe"), "Hello, John Doe!");
    }

    #[test]
    fn test_login() {
        assert!(login("admin", "password"));
        assert!(login("AdmIn", "password"));
        assert!(!login("username2", "password"));
        assert!(!login("admin", "password2"));
    }
}
