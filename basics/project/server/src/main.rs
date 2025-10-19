use axum::{routing::get, Extension, Router};
mod api;
mod collector;
mod commands;
mod web;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Read the .env file and obtain the database URL
    dotenv::dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?;

    // Get a database connection pool
    let pool = sqlx::SqlitePool::connect(&db_url).await?;
    // Run Migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    // Spawn the collector
    let handle = tokio::spawn(collector::data_collector(pool.clone()));

    // Start the web server
    let app = Router::new()
        .route("/", get(web::index))
        .route("/collector.html", get(web::collector))
        .route("/api/all", get(api::show_all))
        .route("/api/collectors", get(api::show_collectors))
        .route("/api/collector/{uuid}", get(api::collector_data))
        .route("/api/collector/{uuid}/shutdown", get(api::shutdown_collector))
        .layer(Extension(pool));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app.into_make_service()).await?;

    // Wait for the data collector to finish
    handle.await??; // Two question marks - we're unwrapping the task result, and the result from running the collector.
    Ok(())
}
