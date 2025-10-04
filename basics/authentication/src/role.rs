use serde::{Deserialize, Serialize};

pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum LoginRole {
    Admin,
    User,
}
