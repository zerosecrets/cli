use rexpect::error::Error;
use rexpect::spawn;

#[test]
fn test_interactive_cli_help() -> Result<(), Error> {
    let mut p = spawn("./target/debug/zero-cli --help", Some(5000))?;
    p.exp_string("CLI for the Zero service")?;
    p.exp_string("Manage teams")?;
    Ok(())
}
