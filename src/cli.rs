use clap::{Parser, ValueHint};

#[derive(Parser, Clone, Debug, Default, PartialEq, Eq)]
pub struct CliOptions {
    /// The path to file with database
    #[clap(value_parser, env, default_value = "database.txt", value_hint = ValueHint::FilePath)]
    pub database_file_path: String,
}
