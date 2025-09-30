use crate::cli::csv_opts::CsvOpts;
use clap::Parser;
use crate::cli::base64_opts::Base64SubCommand;
use crate::cli::http_opts::HttpSubCommand;

pub mod base64_opts;
pub mod csv_opts;
pub mod http_opts;

#[derive(Debug, Parser)]
#[command(name = "rcli", version = "0.1.0", author = "Flint", about = "Rust CLI", long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
    #[command(subcommand)]
    Http(HttpSubCommand),
}
