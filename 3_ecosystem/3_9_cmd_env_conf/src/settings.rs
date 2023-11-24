use config::{Config, ConfigError, Environment, File, FileFormat};
use serde::Deserialize;
use serde_inline_default::serde_inline_default;

#[serde_inline_default]
#[derive(Debug, Clone, Deserialize)]
pub struct Mode {
    #[serde_inline_default(false)]
    pub debug: bool,
}

#[serde_inline_default]
#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    #[serde_inline_default("http://127.0.0.1".to_string())]
    pub external_url: String,
    #[serde_inline_default(8081)]
    pub http_port: u16,
    #[serde_inline_default(8082)]
    pub grpc_port: u16,
    #[serde_inline_default(10025)]
    pub healthz_port: u16,
    #[serde_inline_default(9199)]
    pub metrics_port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Db {
    pub mysql: Mysql,
}

#[serde_inline_default]
#[derive(Debug, Clone, Deserialize)]
pub struct Connections {
    #[serde_inline_default(30)]
    pub max_idle: usize,
    #[serde_inline_default(30)]
    pub max_open: usize,
}

#[serde_inline_default]
#[derive(Debug, Clone, Deserialize)]
pub struct Mysql {
    #[serde_inline_default("127.0.0.1".to_string())]
    pub host: String,
    #[serde_inline_default(3306)]
    pub port: u16,
    #[serde_inline_default("default".to_string())]
    pub dating: String,
    #[serde_inline_default("root".to_string())]
    pub user: String,
    #[serde_inline_default("".to_string())]
    pub pass: String,
    pub connections: Connections,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[serde_inline_default]
#[derive(Debug, Clone, Deserialize)]
pub struct App {
    #[serde_inline_default(LogLevel::Info)]
    pub level: LogLevel,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Log {
    pub app: App,
}

#[serde_inline_default]
#[derive(Debug, Clone, Deserialize)]
pub struct Watchdog {
    #[serde_inline_default("5s".to_string())]
    pub period: String,
    #[serde_inline_default(10)]
    pub limit: usize,
    #[serde_inline_default("4s".to_string())]
    pub lock_timeout: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Background {
    pub watchdog: Watchdog,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub mode: Mode,
    pub server: Server,
    pub db: Db,
    pub log: Log,
    pub background: Background,
}

use crate::cli::{Cli, StructOpt};

impl Settings {
    /// Generates settings from path specified in Cli
    /// and env like CONF_MODE_DEBUG = ...["_" is separator for nested structures]
    pub fn new() -> Result<Self, ConfigError> {
        let opt = Cli::from_args();
        let s = Config::builder()
            // Read from file specified in Cli
            .add_source(File::new(
                &opt.conf
                    .into_os_string()
                    .into_string()
                    .expect("failed to convert path with error:"),
                FileFormat::Toml,
            ))
            // Read from env like CONF_<something nested>_<required field> = ...
            .add_source(Environment::with_prefix("CONF").separator("_"))
            .build()?;
        let mut s: Self = s.try_deserialize()?;
        // Set debug if specified in cli, otherwise don't touch
        if opt.debug {
            s.mode.debug = true;
        }
        Ok(s)
    }
}
