use authentication::{get_users, save_users, LoginRole, User};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command()]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all users.
    List,
    /// Add a user
    Add {
        /// The user's login name
        username: String,
        /// The user's password (plaintext)
        password: String,
        /// Optional - mark as an admin
        #[arg(long)]
        admin: Option<bool>,
    },
    /// Delete a user
    Delete {
        /// User's login to delete
        username: String,
    },
    /// Change a user's password
    ChangePassword {
        /// Username whose password should change
        username: String,
        /// New password
        new_password: String,
    },
}

fn list_users() {
    println!("{:<20} | {:<20}", "Username", "Role");
    println!("{:-<43}", "");

    let users = get_users();
    users.iter().for_each(|(_, user)| {
        println!("{:<20} | {:<20?}", user.username, user.role);
    })
}

fn add_user(username: String, password: String, is_admin: bool) {
    let user = User::new(
        &username,
        &password,
        if is_admin {
            LoginRole::Admin
        } else {
            LoginRole::User
        },
    );

    let mut users = get_users();
    users.insert(username, user);

    save_users(&users);
}

fn delete_user(username: String) {
    let mut users = get_users();
    if !users.contains_key(&username) {
        println!("User {} not found", username);
        return;
    }

    users.remove(&username);

    save_users(&users);
}

fn change_password(username: String, password: String) {
    let mut users = get_users();
    if !users.contains_key(&username) {
        println!("User {} not found", username);
        return;
    }

    let user = users.get(&username).unwrap();
    let new_user = User::new(&username, &password, user.role.clone());

    users.remove(&username);
    users.insert(username, new_user);
    save_users(&users);
}

fn main() {
    let cli = Args::parse();

    match cli.command {
        Some(Commands::List) => {
            list_users();
        }
        Some(Commands::Add {
            username,
            password,
            admin,
        }) => {
            add_user(username, password, admin.unwrap_or(false));
        }
        Some(Commands::Delete { username }) => {
            delete_user(username);
        }
        Some(Commands::ChangePassword {
            username,
            new_password,
        }) => {
            change_password(username, new_password);
        }
        None => println!("Command not provided"),
    }
}
