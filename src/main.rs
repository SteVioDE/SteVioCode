use clap::Parser;

use stevio_code::{cli::Cli, commands, config};

fn main() {
    let cli = Cli::parse();
    let config = match config::load_config() {
        Ok(c) => c,
        Err(e) => {
            panic!("Problem with config: {e:?}");
        }
    };
    commands::handle_command(cli, config).expect("Problem with command");
}
