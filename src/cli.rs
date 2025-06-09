use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version)]
#[command(about = "Manage you git projects.", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// handles projects
    #[command(alias = "p")]
    Projects {
        #[command(subcommand)]
        project_commands: ProjectCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum ProjectCommands {
    /// lists all projects
    List,
}
