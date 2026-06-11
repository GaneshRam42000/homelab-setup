pub trait Docker {
    fn docker_shell_command(&self);
}

pub struct List {
    pub command: String,
}
impl Docker for List {
    fn docker_shell_command(&self) {
        println!("{}", self.command);
    }
}
