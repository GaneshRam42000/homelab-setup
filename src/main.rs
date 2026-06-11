mod docker_interface;
use crate::docker_interface::Docker;
use docker_interface::List;

fn main() {
    let command = List {
        command: String::from("docker container ls"),
    };
    command.docker_shell_command();
}
