use rexpect::error::Error;
use rexpect::spawn;

#[test]
fn test_interactive_cli_help() -> Result<(), Error> {
    let mut p = spawn("cargo run -- help", Some(5000))?;

    p.exp_string("CLI for the Zero service")?;
    p.exp_string("Manage teams")?;
    Ok(())
}

#[test]
fn test_projects_create() -> Result<(), Error> {
    let mut p = spawn("cargo run -- projects create", Some(5000))?;

    p.exp_string("e a project name:")?;
    p.send_line("newone_from_test")?;

    p.exp_string("Do you want to generate a new token for this project?")?;
    p.send_line("n")?;
    p.exp_string("Project successfully created")?;

    // p.send("\x1B[B")?; // v
    // p.send_line("")?; // enter
    Ok(())
}
