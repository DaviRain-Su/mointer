use axum::{routing::get, Router};
use clap::Parser;
use tracing::info;

#[derive(Parser, Debug)]
pub struct Server {
    #[arg(long, default_value = "8080")]
    pub port: u16,
    #[arg(long, default_value = "127.0.0.1")]
    pub host: String,
    #[arg(long)]
    pub workers: Option<usize>,
    #[arg(long)]
    pub threads: Option<usize>,
    #[arg(long)]
    pub keep_alive: Option<usize>,
    #[arg(long)]
    pub keep_alive_timeout: Option<usize>,
}

impl Server {
    pub async fn run(&self) -> anyhow::Result<()> {
        info!("Server running on {}:{}", self.host, self.port);
        // build our application with a single route
        let app = Router::new().route("/", get(|| async { "Hello, World!" }));

        // run our app with hyper, listening globally on port 3000
        let address = format!("{}:{}", self.host, self.port);
        info!("Listening on http://{}", address);
        let listener = tokio::net::TcpListener::bind(address).await?;
        axum::serve(listener, app).await?;
        Ok(())
    }
}
