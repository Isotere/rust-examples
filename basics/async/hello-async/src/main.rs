use futures::executor::block_on;
use futures::future::join_all;
use futures::join;

async fn say_hello() {
    println!("Hello from function!");

    join!(say_world(), say_goodbye());

    let n = double(4).await;
    println!("n: {}", n);

    let n_vec = vec![double(4), double(5)];
    let result = join_all(n_vec).await;
    println!("result: {:?}", result);
}

async fn say_world() {
    println!("Hello from async function!");
}

async fn say_goodbye() {
    println!("Goodbye from function!");
}

async fn double(n: u32) -> u32 {
    n * 2
}

fn main() {
    block_on(say_hello());
}
