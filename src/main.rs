mod db;
mod init;

use actix_web::{App, HttpServer};
use dotenvy::var;
use log::info;
use sqlx::{Pool, Postgres};

struct AppState {
    db_client: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init::load_env_vars();
    init::init_logger();

    // Initialize database client connection
    let db_conn_url = var("DATABASE");
    let db_client = match db_conn_url {
        Ok(url) => db::client::init_db_client(url).await,
        Err(_) => panic!("Database connection url is invalid."),
    };

    // Initialize web server
    let web_server_host = var("WEB_SERVER_HOST").unwrap_or(String::from("0.0.0.0"));
    let web_server_port = var("WEB_SERVER_PORT")
        .unwrap_or(String::from("3000"))
        .parse()
        .unwrap();
    let server = HttpServer::new(move || {
        App::new().app_data(AppState {
            db_client: db_client.clone(),
        })
    })
    .bind((web_server_host.clone(), web_server_port))
    .unwrap();

    info!(
        "Web server listening on '{}:{}'",
        web_server_host, web_server_port
    );
    server.run().await
}
