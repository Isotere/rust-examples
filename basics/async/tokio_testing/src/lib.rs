async fn double(n: u32) -> u32 {
    n * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)] // В скобках необязательно, для примера
    async fn test_double() {
        assert_eq!(double(1).await, 2);
    }
}
