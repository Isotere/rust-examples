async fn hello() -> u32 {
    println!("hello tokio");

    3
}

async fn hello2() -> u32 {
    println!("hello tokio2");

    4
}

async fn ticker() {
    for i in 0..10 {
        println!("tick {}", i);
        tokio::task::yield_now().await;
    }
}

#[tokio::main]
async fn main() {
    hello().await;

    println!();
    let result = tokio::join!(hello(), hello2());
    println!("{:?}", result);

    println!();
    let handle = tokio::spawn(ticker());
    let handle2 = tokio::spawn(hello());

    let _ = tokio::join!(handle, handle2);

    println!();
    let _ = tokio::join!(
        tokio::spawn(hello2()),
        tokio::spawn(hello()),
        tokio::spawn(ticker()),
        tokio::spawn(ticker()),
    );
}
