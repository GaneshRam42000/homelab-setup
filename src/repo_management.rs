use std::fs;
use std::io;
use std::path::PathBuf;

pub struct ComposeProject{
    pub name: String,
    pub path: PathBuf,
    pub file_path: PathBuf,
    pub env_path: PathBuf,
}

pub struct ComposeRepository{
    pub path: PathBuf,
}

impl ComposeRepository {
    pub fn new(path: PathBuf) -> Self{
        Self { path }
    }
    pub fn projects(&self) -> io::Result<Vec<ComposeProject>> {
        let mut projects = Vec::new();
        for entry in fs::read_dir(&self.path)? {
            let entry = entry?;
            let project_path = entry.path();
            if !project_path.is_dir(){
                continue;
            }
            let compose_file = project_path.join("docker-compose.yaml");
            let env_file = project_path.join(".env");
            if !compose_file.is_file() && !env_file.is_file() {
                continue;
            }
            let name = entry.file_name().to_string_lossy().into_owned();
            projects.push(ComposeProject { name, path: (project_path), file_path: (compose_file), env_path: (env_file) });
        }

        Ok(projects)
    }
}
