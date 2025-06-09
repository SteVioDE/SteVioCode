use std::path::Path;

use walkdir::WalkDir;

use crate::{
    config::Config,
    core::{
        project_detectors::{ProjectDetector, default_detectors},
        types::{ProjectInfo, ProjectType},
    },
};

pub struct ProjectManager {
    config: Config,
    detectors: Vec<Box<dyn ProjectDetector>>,
}

impl ProjectManager {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            detectors: default_detectors(),
        }
    }

    pub fn with_detectors(config: Config, detectors: Vec<Box<dyn ProjectDetector>>) -> Self {
        Self { config, detectors }
    }

    pub fn scan_projects(&self) -> Result<Vec<ProjectInfo>, Box<dyn std::error::Error>> {
        self.find_projects_in_path(&self.config.projects_path)
    }

    fn find_projects_in_path<P: AsRef<Path>>(
        &self,
        root_path: P,
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

            // TODO: update this later
            // we wanna support git submodules and nested projects
            if entry.file_type().is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.starts_with('.') && name != ".git" {
                        continue;
                    }
                }

                if let Some(project_info) = self.detect_project(entry.path()) {
                    if self.should_skip_recursion(&project_info) {
                        skip_depth = Some(depth);
                    }
                    projects.push(project_info);
                }
            }
        }
        Ok(projects)
    }

    fn detect_project(&self, path: &Path) -> Option<ProjectInfo> {
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
