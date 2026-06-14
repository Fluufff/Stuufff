use clap::Subcommand;

use crate::config::error::RuntimeError;

pub mod migrate;

#[derive(Subcommand)]
pub enum Command {
    /// Migrate database scheme to latest version
    Migrate {
        #[command(flatten)]
        input: migrate::RunInput,
    },
}

pub async fn run(
    base: &mut clap::builder::Command,
    cmd: Option<Command>,
) -> Result<(), RuntimeError> {
    match cmd {
        None => {
            base.print_help().unwrap();
            Err(RuntimeError::NoRunnerSpecified)
        }
        Some(cmd) => match cmd {
            Command::Migrate { input } => {
                migrate::run(input).await?;
                Ok(())
            }
        },
    }
}
