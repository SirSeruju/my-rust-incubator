use std::path::PathBuf;
pub use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "step_3_9", about = "Prints its configuration to STDOUT.")]
pub struct Cli {
    /// Enables debug mode
    #[structopt(short, long)]
    pub debug: bool,

    /// Path to configuration file [env: CONF_FILE=]  [default: config.toml]
    #[structopt(
        short,
        long,
        parse(from_os_str),
        default_value = "config.toml",
        env = "CONF_FILE"
    )]
    pub conf: PathBuf,
}
