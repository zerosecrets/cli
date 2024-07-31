use rexpect::error::Error;
use rexpect::spawn;

// FIXME change P variable for more wordy
#[test]
fn test_secrets_create_with_one_field() -> Result<(), Error> {
    let mut p = spawn("cargo run -- secrets create --id dc1c", Some(15000))?;
    p.exp_string("Type a name for the secret")?;
    p.send_line("test secret one field")?;

    p.send("\x1B[B")?;
    p.send("\x1B[B")?;
    p.send("\x1B[B")?;
    p.send("\x1B[B")?;
    p.send_line("")?;

    p.exp_string("Type a field name:")?;
    p.send_line("new_secret_one_field")?;
    p.exp_string("Type a field value:")?;
    p.send_line("111222")?;
    p.exp_string("Do you want to add another field")?;
    p.send_line("n")?;
    p.exp_string("Secret link:")?;
    p.exp_string("Secret ID:")?;
    Ok(())
}

#[test]
fn test_secrets_create_with_many_fields() -> Result<(), Error> {
    let mut p = spawn("cargo run -- secrets create --id dc1c", Some(15000))?;
    p.exp_string("Type a name for the secret")?;
    p.send_line("new_secret_many_fields")?;

    p.send("\x1B[B")?;
    p.send("\x1B[B")?;
    p.send("\x1B[B")?;
    p.send_line("")?;

    p.exp_string("Type a field name:")?;
    p.send_line("first")?;
    p.exp_string("Type a field value:")?;
    p.send_line("11111")?;
    p.exp_string("Do you want to add another field")?;
    p.send_line("y")?;
    p.exp_string("Type a field name:")?;
    p.send_line("second")?;
    p.exp_string("Type a field value:")?;
    p.send_line("222222")?;
    p.exp_string("Do you want to add another field")?;
    p.send_line("y")?;
    p.exp_string("Type a field name:")?;
    p.send_line("third")?;
    p.exp_string("Type a field value:")?;
    p.send_line("3333")?;
    p.exp_string("Do you want to add another field")?;
    p.send_line("n")?;
    p.exp_string("Secret link:")?;
    p.exp_string("Secret ID:")?;
    Ok(())
}

#[test]
fn test_secrets_delete() -> Result<(), Error> {
    let mut p = spawn("cargo run -- secrets delete --id fc2d", Some(15000))?;
    p.exp_string("Type fc2d to confirm deletion")?;
    p.send_line("fc2d")?;
    p.exp_string("Secret successfully deleted")?;
    Ok(())
}

#[test]
fn test_secrets_edit() -> Result<(), Error> {
    let mut p = spawn("cargo run -- secrets edit --id fc1d5", Some(15000))?;
    p.exp_string("Type a new secret name")?;
    p.send_line("edited secret")?;
    p.send("\x1B[B")?;
    p.send_line("")?;

    p.exp_string("The secret has been successfully updated.")?;
    Ok(())
}

#[test]
fn test_secrets_list() -> Result<(), Error> {
    let mut p = spawn("cargo run -- secrets list --id dc1c", Some(15000))?;
    p.exp_string("#fc3d")?;
    p.exp_string("list secret")?;
    Ok(())
}

#[test]
fn test_secrets_share() -> Result<(), Error> {
    let mut p = spawn("cargo run -- secrets share --id fc4d", Some(15000))?;
    p.exp_string("Type a passphrase of at least 6 character")?;
    p.send_line("123Qwe")?;
    p.exp_string("Expires in")?;
    p.send("\x1B[B")?;
    p.send_line("")?;
    p.exp_string("Pick secret fields")?;
    p.send_line(" ")?;
    p.send_line("")?;
    p.exp_string("Your link for the shared secret")?;
    Ok(())
}

#[test]
fn test_secrets_view() -> Result<(), Error> {
    let mut p = spawn("cargo run -- secrets view --id fc4d", Some(15000))?;
    p.exp_string("URL")?;
    p.exp_string("Name")?;
    p.exp_string("Vendor")?;
    p.exp_string("Fields")?;
    Ok(())
}
