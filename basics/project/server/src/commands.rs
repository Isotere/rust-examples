use shared_data::TaskType;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

static COMMANDS: LazyLock<Mutex<HashMap<u128, TaskType>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

pub fn add_command(collector_id: u128, command: TaskType) {
    let mut commands = COMMANDS.lock().unwrap();
    commands.insert(collector_id, command);
}

pub fn get_commands(collector_id: u128) -> Option<TaskType> {
    let mut commands = COMMANDS.lock().unwrap();
    commands.remove(&collector_id)
}
