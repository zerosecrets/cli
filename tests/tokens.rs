use rexpect::error::Error;
use rexpect::spawn;

#[test]
fn test_tokens_create() -> Result<(), Error> {
    let mut p = spawn("cargo run -- tokens create --id dc1c", Some(15000))?;
    p.exp_string("Type a name for the token")?;
    p.send_line("project_token")?;
    p.exp_string("Expires in")?;
    p.exp_string("Endless")?;
    p.send_line("")?;
    p.exp_string("Token successfully created")?;
    p.exp_string("Name")?;
    p.exp_string("Expires at")?;
    Ok(())
}

#[test]
fn test_tokens_delete() -> Result<(), Error> {
    let mut p = spawn("cargo run -- tokens delete --id d1bd", Some(15000))?;
    p.exp_string("confirm deletion")?;
    p.send_line("d1bd")?;
    p.exp_string("Token successfully deleted")?;
    Ok(())
}

#[test]
fn test_tokens_list() -> Result<(), Error> {
    let mut p = spawn("cargo run -- tokens list --id dc1c", Some(15000))?;
    p.exp_string("Tokens of the project")?;
    p.exp_string("Use <Esc>/<q> to exit")?;
    p.exp_string("#d2bd")?;
    p.exp_string("list_token")?;
    Ok(())
}

#[test]
fn test_tokens_regenerate_endless() -> Result<(), Error> {
    let mut p = spawn("cargo run -- tokens regenerate --id d2bd", Some(15000))?;
    p.exp_string("Expires in")?;
    p.exp_string("Endless")?;
    p.send_line("")?;
    p.exp_string("Token successfully regenerated")?;
    Ok(())
}

#[test]
fn test_tokens_regenerate_7_days() -> Result<(), Error> {
    let mut p = spawn("cargo run -- tokens regenerate --id d2bd", Some(15000))?;
    p.exp_string("Expires in")?;
    p.exp_string("Endless")?;
    p.send("\x1B[B")?;
    p.send_line("")?;
    p.exp_string("Token successfully regenerated")?;
    p.exp_string("7 days")?;
    Ok(())
}