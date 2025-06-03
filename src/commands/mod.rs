use crate::{
    cli::{Cli, Commands},
    config::Config,
};

mod project;

pub fn handle_command(cli: Cli, config: Config) {
    match cli.command {
        Some(command) => match command {
            Commands::Projects { project_commands } => project::handle(project_commands),
        },
        None => todo!(),
    }
}
