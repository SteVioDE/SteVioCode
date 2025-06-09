use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
    path::PathBuf,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProjectType {
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
pub struct ProjectInfo {
    pub path: PathBuf,
    pub project_types: HashSet<ProjectType>,
}

impl ProjectInfo {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            project_types: HashSet::new(),
        }
    }

    pub fn add_type(&mut self, project_type: ProjectType) {
        self.project_types.insert(project_type);
    }

    pub fn has_type(&self, project_type: &ProjectType) -> bool {
        self.project_types.contains(project_type)
    }
}
