mod init;

fn main() {
    init::load_env_vars();
    init::init_logger();
}
