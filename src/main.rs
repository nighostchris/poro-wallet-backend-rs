use dotenvy::dotenv;

fn load_env_vars() {
    match dotenv() {
        Ok(_) => println!("Successfully loaded environment variables."),
        Err(_) => panic!("Failed to load environment variables from '.env' file."),
    }
}

fn main() {
    load_env_vars();
}
