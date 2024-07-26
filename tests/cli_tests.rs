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
    p.exp_string("project name:")?;
    p.send_line("newone_from_test_2321")?;
    p.exp_string("Do you want to generate a new token for this project?")?;
    p.send_line("n")?;
    p.exp_string("Project successfully created")?;
    Ok(())
}

// FIXME add project id static
#[test]
fn test_secrets_create_with_one_field() -> Result<(), Error> {
    // TODO interactions with select
    let mut p = spawn("cargo run -- secrets create --id 8b8c", Some(15000))?;
    p.exp_string("Type a name for the secret")?;
    p.send_line("test secret one field")?;

    // 4 down Bibuket
    p.send("\x1B[B")?; // v
    p.send("\x1B[B")?; // v
    p.send("\x1B[B")?; // v
    p.send("\x1B[B")?; // v
    p.send_line("")?; // enter

    p.exp_string("Type a field name:")?;
    p.send_line("secretok")?;
    p.exp_string("Type a field value:")?;
    p.send_line("123412")?;
    p.exp_string("Do you want to add another field")?;
    p.send_line("n")?;
    p.exp_string("Secret link:")?;
    p.exp_string("Secret ID:")?;
    Ok(())
}

// FIXME add project id static
#[test]
fn test_secrets_create_with_many_fields() -> Result<(), Error> {
    // TODO interactions with select
    let mut p = spawn("cargo run -- secrets create --id 8b8c", Some(15000))?;
    p.exp_string("Type a name for the secret")?;
    p.send_line("test secret many fields")?;

    p.send("\x1B[B")?; // v
    p.send("\x1B[B")?; // v
    p.send("\x1B[B")?; // v
    p.send_line("")?; // enter

    p.exp_string("Type a field name:")?;
    p.send_line("first")?;
    p.exp_string("Type a field value:")?;
    p.send_line("123412")?;
    p.exp_string("Do you want to add another field")?;
    p.send_line("y")?;
    p.exp_string("Type a field name:")?;
    p.send_line("second")?;
    p.exp_string("Type a field value:")?;
    p.send_line("abc abc 123 qwe")?;
    p.exp_string("Do you want to add another field")?;
    p.send_line("y")?;
    p.exp_string("Type a field name:")?;
    p.send_line("third")?;
    p.exp_string("Type a field value:")?;
    p.send_line("abc abc 123")?;
    p.exp_string("Do you want to add another field")?;
    p.send_line("n")?;
    p.exp_string("Secret link:")?;
    p.exp_string("Secret ID:")?;
    Ok(())
}

#[test]
fn test_secrets_delete() -> Result<(), Error> {
    // TODO interactions with select
    let mut p = spawn("cargo run -- secrets delete --id 9599", Some(15000))?;
    p.exp_string("Type 9599 to confirm deletion")?;
    p.send_line("9599")?;
    p.exp_string("Secret successfully deleted")?;
    Ok(())
}
