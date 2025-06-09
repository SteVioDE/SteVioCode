use std::path::Path;

use crate::core::types::ProjectType;

pub trait ProjectDetector {
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

pub fn default_detectors() -> Vec<Box<dyn ProjectDetector>> {
    vec![
        Box::new(GitDetector),
        Box::new(RustDetector),
        Box::new(JavaDetector),
        Box::new(GoDetector),
    ]
}
