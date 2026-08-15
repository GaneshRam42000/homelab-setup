use std::process::Command;

pub struct List;

impl List {
    pub fn run(&self) -> std::io::Result<()> {
        Command::new("docker").args(["container", "ls"]).status()?;
        Ok(())
    }
}

pub struct ComposeStart {
    pub compose_file_path: String,
    pub compose_env_path: String,
}

impl ComposeStart {
    pub fn run(&self) -> std::io::Result<()> {
        Command::new("docker")
            .args([
                "compose",
                "-f",
                &self.compose_file_path,
                "--env-file",
                &self.compose_env_path,
                "up"
            ])
            .status()?;
        Ok(())
    }
}

pub struct ComposeStop {
    pub compose_file_path: String,
}

impl ComposeStop {
    pub fn run(&self) -> std::io::Result<()> {
        Command::new("docker")
            .args(["compose", "-f", &self.compose_file_path, "stop"])
            .status()?;
        Ok(())
    }
}

pub struct ComposeRestart {
    pub compose_file_path: String,
}

impl ComposeRestart {
    pub fn run(&self) -> std::io::Result<()> {
        Command::new("docker")
            .args(["compose", "-f", &self.compose_file_path, "restart"])
            .status()?;
        Ok(())
    }
}

pub struct ComposePrune {
    pub compose_file_path: String,
}

impl ComposePrune {
    pub fn run(self) -> std::io::Result<()> {
        Command::new("docker")
            .args(["compose", "-f", &self.compose_file_path, "down"])
            .status()?;
        Ok(())
    }
}
