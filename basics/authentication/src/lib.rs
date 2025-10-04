mod role;

pub use role::*;
use std::collections::HashMap;
use std::path::Path;

mod user;
pub use user::*;

mod hashing;
pub use hashing::*;

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let users = get_users();

    let username = username.to_lowercase();

    let user = users.get(&username)?;

    if hash_password(password) == user.password {
        return Some(LoginAction::Granted(user.role.clone())); // Либо можем использовать into_iter, который делает move из вектора
    }

    Some(LoginAction::Denied)
}

// pub fn get_users() -> Vec<User> {
//     vec![
//         User::new("admin", "password", LoginRole::Admin),
//         User::new("user", "u_password", LoginRole::User),
//     ]
// }

pub fn get_default_users() -> HashMap<String, User> {
    let mut users = HashMap::new();
    users.insert(
        "admin".to_string(),
        User::new("admin", "password", LoginRole::Admin),
    );
    users.insert(
        "user".to_string(),
        User::new("user", "u_password", LoginRole::User),
    );

    users
}

pub fn get_users() -> HashMap<String, User> {
    let users_path = Path::new("users.json");

    if users_path.exists() {
        let users_json = std::fs::read_to_string(users_path).expect("Unable to read users.json");
        let users = serde_json::from_str::<HashMap<String, User>>(&users_json)
            .expect("Unable to parse users.json");

        return users;
    }

    let users = get_default_users();
    let users_json = serde_json::to_string_pretty(&users).expect("Failed to serialize users.json");
    std::fs::write(users_path, users_json).expect("Failed to write users.json");

    users
}

// pub fn get_admin_users() -> Vec<String> {
//     let users: Vec<String> = get_users()
//         .into_iter()
//         .filter(|e| )
//         .map(|user| user.username)
//         .collect();
//
//     users
// }

#[cfg(test)]
mod tests {
    #[test]
    fn test_login() {}
}
