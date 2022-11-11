mod init;

use actix_web::{App, HttpServer};
use dotenvy::var;
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init::load_env_vars();
    init::init_logger();

    let web_server_host = var("WEB_SERVER_HOST").unwrap_or(String::from("0.0.0.0"));
    let web_server_port = var("WEB_SERVER_PORT")
        .unwrap_or(String::from("3000"))
        .parse()
        .unwrap();
    let server = HttpServer::new(move || App::new())
        .bind((web_server_host.clone(), web_server_port))
        .unwrap();

    info!(
        "Web server listening on '{}:{}'",
        web_server_host, web_server_port
    );
    server.run().await
}
