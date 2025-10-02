pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(Debug, PartialEq)]
pub enum LoginRole {
    Admin,
    User,
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username = username.to_lowercase();

    if username != "admin" && username != "user" {
        return None;
    }

    if username == "admin" && password == "password" {
        return Some(LoginAction::Granted(LoginRole::Admin));
    }

    if username == "user" && password == "u_password" {
        return Some(LoginAction::Granted(LoginRole::User));
    }

    Some(LoginAction::Denied)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_login() {}
}
