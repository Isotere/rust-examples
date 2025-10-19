use axum::response::Html;

pub async fn index() -> Html<String> {
    let path = std::path::Path::new("html/index.html");
    let content = tokio::fs::read_to_string(path).await.unwrap();
    Html(content)
}

pub async fn collector() -> Html<String> {
    let path = std::path::Path::new("html/collector.html");
    let content = tokio::fs::read_to_string(path).await.unwrap();
    Html(content)
}
