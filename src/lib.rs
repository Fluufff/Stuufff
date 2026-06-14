use clap::CommandFactory;

pub mod cli;
pub mod commands;
pub mod config;
pub mod psql;
pub mod version;

pub async fn run() -> Result<(), String> {
    let cli = cli::parse();
    let mut cmd = cli::Args::command();

    match cli.command {
        None => {
            cmd.print_help().unwrap();
            Err("no command given".into())
        }
        Some(cli::Commands::Version { short }) => {
            version::print_version(short);
            Ok(())
        }
        Some(cli::Commands::Api { input }) => {
            commands::api::run(input).await.map_err(|e| e.to_string())
        }
        Some(cli::Commands::Db { db }) => {
            let base = cmd.find_subcommand_mut("db").unwrap();
            commands::db::run(base, db).await.map_err(|e| e.to_string())
        }
    }
}
