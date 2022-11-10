use chrono::Local;
use dotenvy::dotenv;
use env_logger::{Builder, Env};
use log::info;
use std::io::Write;

pub fn load_env_vars() {
    match dotenv() {
        Ok(_) => println!("Successfully loaded environment variables."),
        Err(_) => panic!("Failed to load environment variables from '.env' file."),
    }
}

pub fn init_logger() {
    let env = Env::default().filter_or("LOG_LEVEL", "debug");

    Builder::from_env(env)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}][{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.args()
            )
        })
        .init();

    info!("Successfully setup logger.")
}
