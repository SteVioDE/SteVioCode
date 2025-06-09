use crate::prelude::*;

pub fn handle(
    project_command: ProjectCommands,
    config: Config,
) -> Result<(), Box<dyn std::error::Error>> {
    match project_command {
        ProjectCommands::List => {
            list_projects(config)?;
        }
    }
    Ok(())
}

fn list_projects(config: Config) -> Result<Vec<ProjectInfo>, Box<dyn std::error::Error>> {
    let pm = ProjectManager::new(config);
    let projects = pm.scan_projects()?;

    for project in &projects {
        let types: Vec<String> = project
            .project_types
            .iter()
            .map(|t| t.to_string())
            .collect();
        println!("{} [{}]", project.path.display(), types.join(", "));
    }
    Ok(projects)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;

    use super::*;

    fn create_test_config(temp_dir: &TempDir) -> Config {
        Config {
            projects_path: temp_dir.path().to_string_lossy().to_string(),
        }
    }

    fn create_test_project_structure(temp_dir: &TempDir) -> Result<(), std::io::Error> {
        let base_path = temp_dir.path();

        let rust_project = base_path.join("rust_project");
        fs::create_dir_all(&rust_project)?;
        fs::write(rust_project.join("Cargo.toml"), "")?;

        let git_project = base_path.join("git_project");
        fs::create_dir_all(&git_project)?;
        fs::create_dir_all(git_project.join(".git"))?;

        let go_project = base_path.join("go_project");
        fs::create_dir_all(&go_project)?;
        fs::write(go_project.join("go.mod"), "")?;

        let maven_project = base_path.join("maven_project");
        fs::create_dir_all(&maven_project)?;
        fs::write(maven_project.join("pom.xml"), "")?;

        let gradle_project = base_path.join("gradle_project");
        fs::create_dir_all(&gradle_project)?;
        fs::write(gradle_project.join("build.gradle"), "")?;

        let multi_project = base_path.join("multi_project");
        fs::create_dir_all(&multi_project)?;
        fs::write(multi_project.join("Cargo.toml"), "")?;
        fs::create_dir_all(multi_project.join(".git"))?;

        Ok(())
    }

    #[test]
    fn test_handle_list_command() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        create_test_project_structure(&temp_dir).expect("Failed to create test structure");
        let config = create_test_config(&temp_dir);

        let result = handle(ProjectCommands::List, config);
        assert!(result.is_ok(), "handle should succeed for List command");
    }

    #[test]
    fn test_list_projects_basic() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        create_test_project_structure(&temp_dir).expect("Failed to create test structure");
        let config = create_test_config(&temp_dir);

        let result = list_projects(config);
        assert!(result.is_ok(), "list_projects should succeed");

        let projects = result.unwrap();
        assert!(!projects.is_empty(), "Should find at least one project");

        let project_paths: Vec<String> = projects
            .iter()
            .map(|p| p.path.file_name().unwrap().to_string_lossy().to_string())
            .collect();

        assert!(project_paths.contains(&"rust_project".to_string()));
        assert!(project_paths.contains(&"git_project".to_string()));
        assert!(project_paths.contains(&"go_project".to_string()));
        assert!(project_paths.contains(&"maven_project".to_string()));
        assert!(project_paths.contains(&"gradle_project".to_string()));
        assert!(project_paths.contains(&"multi_project".to_string()));
    }

    #[test]
    fn test_list_projects_with_types() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        create_test_project_structure(&temp_dir).expect("Failed to create test structure");
        let config = create_test_config(&temp_dir);

        let projects = list_projects(config).expect("list_projects should succeed");

        let multi_project = projects
            .iter()
            .find(|p| p.path.file_name().unwrap() == "multi_project")
            .expect("Should find multi_project");

        assert!(multi_project.has_type(&ProjectType::Rust));
        assert!(multi_project.has_type(&ProjectType::Git));

        let rust_project = projects
            .iter()
            .find(|p| p.path.file_name().unwrap() == "rust_project")
            .expect("Should find rust_project");

        assert!(rust_project.has_type(&ProjectType::Rust));
        assert!(!rust_project.has_type(&ProjectType::Git));
    }

    #[test]
    fn test_list_projects_nonexistent_directory() {
        let config = Config {
            projects_path: "/not/existent/directory".to_string(),
        };

        let result = list_projects(config);
        assert!(
            result.is_err(),
            "Should return error for nonexistent directory"
        );
    }
}
