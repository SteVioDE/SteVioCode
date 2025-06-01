use crate::cli::{Cli, Commands};

mod project;

pub fn handle_command(cli: Cli) {
    match cli.command {
        Some(command) => match command {
            Commands::Projects { project_commands } => project::handle(project_commands),
        },
        None => todo!(),
    }
}
