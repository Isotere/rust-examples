use tokio::task::spawn_blocking;

async fn hello_delay(task: u64, delay: u64) {
    println!("Task {task} has started");

    // because spawn_blocking is often used for various kinds of IO operations that cannot be performed asynchronously
    let _ = spawn_blocking(move || {
        std::thread::sleep(std::time::Duration::from_millis(delay));
    })
    .await;
    println!("Task {task} has finished");
}

#[tokio::main]
async fn main() {
    tokio::join!(hello_delay(1, 500), hello_delay(2, 1000), hello_delay(3, 100),);
}
