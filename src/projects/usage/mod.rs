/// ## Projects Usage Details
///
///  The `usage details` function displays information about the specific usage.
///  Outputs detailed information about the record to the terminal.
///
mod details;
/// ## Projects Usage List
///
///  The `usage list` function displays information about the use of the project without indicating
///  the requested secrets, but will indicate their count.
///
mod list;
use crate::projects::usage::details::{usage_details, ProjectsUsageDetailsArgs};
use crate::projects::usage::list::{usage, ProjectsUsageListArgs};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct ProjectsUsageCommands {
    #[clap(subcommand)]
    commands: ProjectsUsageCommandsEnum,
}

#[derive(Subcommand, Debug)]
#[clap(about = "View a project usage")]
pub enum ProjectsUsageCommandsEnum {
    #[clap(about = "View a project usage details")]
    Details(ProjectsUsageDetailsArgs),
    #[clap(about = "List all project usages")]
    List(ProjectsUsageListArgs),
}

pub fn usage_commands(input: &ProjectsUsageCommands) {
    match &input.commands {
        ProjectsUsageCommandsEnum::Details(args) => usage_details(&args),
        ProjectsUsageCommandsEnum::List(args) => usage(&args),
    }
}
