use clap::Parser;
use cli::Cli;

mod cli;
mod commands;

fn main() {
    let cli = Cli::parse();
    commands::handle_command(cli);
}
