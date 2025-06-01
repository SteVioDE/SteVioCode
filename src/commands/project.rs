use crate::cli::ProjectCommands;

pub fn handle(project_command: Option<ProjectCommands>) {
    println!("{:?}", project_command);
}
