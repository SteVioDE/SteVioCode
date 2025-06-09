use std::path::PathBuf;

use stevio_code::errors::ScError;

use crate::core::types::ProjectInfo;

pub struct ProjectStorage {
    db_path: PathBuf,
}

impl ProjectStorage {
    pub fn load_projects(&self) -> Result<Vec<ProjectInfo>, ScError> {
        !todo!()
    }

    pub fn save_projects(&self, projects: &[ProjectInfo]) -> Result<(), ScError> {
        !todo!()
    }
}
