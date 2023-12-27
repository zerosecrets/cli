/// ## Teams User Invite
///
///  The `user-invite` function allows users to send invitations to a team.
///
mod invite;
/// ## Teams User Remove
///
///  The `user remove` function is responsible for deleting the user from the team of Zero platform.
///  Prompt the user to confirm deletion, after receiving confirmation remove the user-team association
///  Display a success message.
///
mod remove;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct TeamsUserCommands {
    #[clap(subcommand)]
    commands: TeamsUserCommandsEnum,
}

#[derive(Subcommand, Debug)]
#[clap(about = "Manage team users")]
pub enum TeamsUserCommandsEnum {
    #[clap(about = "Invite a user to a team")]
    Invite(invite::UserInviteArgs),
    #[clap(about = "Remove a user from a team")]
    Remove(remove::UserRemoveArgs),
}

pub fn teams_user_commands(input: &TeamsUserCommands) {
    match &input.commands {
        TeamsUserCommandsEnum::Invite(args) => invite::invite(&args),
        TeamsUserCommandsEnum::Remove(args) => remove::remove(&args),
    }
}
