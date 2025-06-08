use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
    path::{Path, PathBuf},
};

use crate::{cli::ProjectCommands, config::Config};
use walkdir::WalkDir;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ProjectType {
    Git,
    Rust,
    Java,
    Go,
}

impl Display for ProjectType {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            ProjectType::Git => write!(f, "Git"),
            ProjectType::Rust => write!(f, "Rust"),
            ProjectType::Java => write!(f, "Java"),
            ProjectType::Go => write!(f, "Go"),
        }
    }
}

#[derive(Debug, Clone)]
struct ProjectInfo {
    path: PathBuf,
    project_types: HashSet<ProjectType>,
}

impl ProjectInfo {
    fn new(path: PathBuf) -> Self {
        Self {
            path,
            project_types: HashSet::new(),
        }
    }

    fn add_type(&mut self, project_type: ProjectType) {
        self.project_types.insert(project_type);
    }

    fn has_type(&self, project_type: &ProjectType) -> bool {
        self.project_types.contains(project_type)
    }
}

trait ProjectDetector {
    fn detect(&self, path: &Path) -> bool;
    fn project_type(&self) -> ProjectType;
}

struct GitDetector;
impl ProjectDetector for GitDetector {
    fn detect(&self, path: &Path) -> bool {
        let git_path = path.join(".git");
        git_path.exists() && (git_path.is_dir() || git_path.is_file())
    }

    fn project_type(&self) -> ProjectType {
        ProjectType::Git
    }
}

struct RustDetector;
impl ProjectDetector for RustDetector {
    fn detect(&self, path: &Path) -> bool {
        path.join("Cargo.toml").exists()
    }

    fn project_type(&self) -> ProjectType {
        ProjectType::Rust
    }
}

pub struct JavaDetector;
impl ProjectDetector for JavaDetector {
    fn detect(&self, path: &Path) -> bool {
        path.join("pom.xml").exists()
            || path.join("build.gradle").exists()
            || path.join("build.gradle.kts").exists()
    }

    fn project_type(&self) -> ProjectType {
        ProjectType::Java
    }
}

pub struct GoDetector;
impl ProjectDetector for GoDetector {
    fn detect(&self, path: &Path) -> bool {
        path.join("go.mod").exists()
    }

    fn project_type(&self) -> ProjectType {
        ProjectType::Go
    }
}

struct ProjectScanner {
    detectors: Vec<Box<dyn ProjectDetector>>,
}

impl ProjectScanner {
    fn new() -> Self {
        Self {
            detectors: vec![
                Box::new(GitDetector),
                Box::new(RustDetector),
                Box::new(JavaDetector),
                Box::new(GoDetector),
            ],
        }
    }

    // fn with_detectors(detectors: Vec<Box<dyn ProjectDetector>>) -> Self {
    //     Self { detectors }
    // }

    fn detect_projects(&self, path: &Path) -> Option<ProjectInfo> {
        let mut project_info = ProjectInfo::new(path.to_path_buf());
        let mut found_any = false;
        for detector in &self.detectors {
            if detector.detect(path) {
                project_info.add_type(detector.project_type());
                found_any = true;
            }
        }

        if found_any { Some(project_info) } else { None }
    }

    fn should_skip_recursion(&self, project_info: &ProjectInfo) -> bool {
        project_info.has_type(&ProjectType::Git)
    }
}

fn find_folders<P: AsRef<Path>>(
    root_path: P,
    scanner: &ProjectScanner,
) -> Result<Vec<ProjectInfo>, Box<dyn std::error::Error>> {
    let mut projects = Vec::new();
    let mut skip_depth = None;

    for entry in WalkDir::new(root_path).into_iter() {
        let entry = entry?;
        let depth = entry.depth();

        if let Some(skip_until_depth) = skip_depth {
            if depth > skip_until_depth {
                continue;
            } else {
                skip_depth = None;
            }
        }

        if entry.file_type().is_dir() {
            if let Some(name) = entry.file_name().to_str() {
                if name.starts_with('.') && name != ".git" {
                    continue;
                }
            }

            if let Some(project_info) = scanner.detect_projects(entry.path()) {
                if scanner.should_skip_recursion(&project_info) {
                    skip_depth = Some(depth);
                }
                projects.push(project_info);
            }
        }
    }
    Ok(projects)
}

pub fn handle(
    project_command: Option<ProjectCommands>,
    config: Config,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{:?}", project_command);
    println!("{:?}", config);

    let scanner = ProjectScanner::new();
    let projects = find_folders(&config.projects_path, &scanner)?;

    for project in projects {
        let types: Vec<String> = project
            .project_types
            .iter()
            .map(|t| t.to_string())
            .collect();
        println!("{} [{}]", project.path.display(), types.join(", "));
    }
    Ok(())
}
