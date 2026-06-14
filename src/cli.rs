use super::commands;
use clap::{Parser, Subcommand};

use crate::config::db::{
    DB_DEFAULT_HOST, DB_DEFAULT_NAME, DB_DEFAULT_PASS, DB_DEFAULT_PORT, DB_DEFAULT_USER,
};

pub fn parse() -> Args {
    Args::parse()
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Print version info
    Version {
        #[arg(short, long)]
        short: bool,
    },
    /// Run the API
    Api {
        #[command(flatten)]
        input: commands::api::RunInput,
    },
    /// Interact with the database
    Db {
        #[command(subcommand)]
        db: Option<commands::db::Command>,
    },
}

#[derive(Debug, clap::Args, Clone)]
pub struct DBArgs {
    #[arg(long, help = &format!("Database host\t\tdefault: \"{DB_DEFAULT_HOST}\""))]
    pub db_host: Option<String>,

    #[arg(long, help = &format!("Database port\t\tdefault: \"{DB_DEFAULT_PORT}\""))]
    pub db_port: Option<String>,

    #[arg(long, help = &format!("Database name\t\tdefault: \"{DB_DEFAULT_NAME}\""))]
    pub db_name: Option<String>,

    #[arg(long, help = &format!("Database user\t\tdefault: \"{DB_DEFAULT_USER}\""))]
    pub db_user: Option<String>,

    #[arg(long, help = &format!("Database pass\t\tdefault: \"{DB_DEFAULT_PASS}\""))]
    pub db_pass: Option<String>,
}
