mod docker_interface;
use docker_interface::List;

fn main() -> std::io::Result<()> {
    let command = List;
    command.run()?;
    Ok(())
}
