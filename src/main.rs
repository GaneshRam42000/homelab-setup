mod docker_interface;
mod repo_management;
use std::path::PathBuf;

use docker_interface::List;
use repo_management::ComposeRepository;
fn main() -> std::io::Result<()> {
    let repo = ComposeRepository::new(PathBuf::from(""));
    Ok(())
}
