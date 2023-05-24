use proc_macros::DisplayUsingDebug;
use serde::{Deserialize, Serialize};
use strum::Display;
use tracing::{level_filters::LevelFilter, subscriber::set_global_default};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::SubscriberBuilder;

#[derive(
    Default, Debug, Copy, Clone, Display, Eq, PartialEq, Ord, PartialOrd, Deserialize, Serialize,
)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

#[derive(
    Default, Debug, Display, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Deserialize, Serialize,
)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    All,
    Compact,
    Pretty,
    #[default]
    Json,
}

#[derive(Debug, Default, DisplayUsingDebug, Copy, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct Config {
    level: Level,
    mode: Mode,
}

pub fn init(config: Config) {
    let is_local_env = crate::config::is_local_env();
    LogTracer::init().expect("failed to set logger");

    let subscriber_builder = tracing_subscriber::fmt()
        .with_level(true)
        .with_ansi(is_local_env)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .with_max_level(config.level);

    _finish(subscriber_builder, config.mode);
}

fn _finish(builder: SubscriberBuilder, mode: Mode) {
    match mode {
        Mode::All => {
            let builder = builder.finish();
            set_global_default(builder).expect("failed to set subscriber")
        },
        Mode::Compact => {
            let builder = builder.compact().finish();
            set_global_default(builder).expect("failed to set subscriber")
        },
        Mode::Pretty => {
            let builder = builder.pretty().finish();
            set_global_default(builder).expect("failed to set subscriber")
        },
        Mode::Json => {
            let builder = builder.json().finish();
            set_global_default(builder).expect("failed to set subscriber")
        },
    }
}

impl Config {
    pub fn new(min_level: Level, mode: Mode) -> Self {
        Self {
            level: min_level,
            mode,
        }
    }
}

impl From<Level> for LevelFilter {
    fn from(l: Level) -> Self {
        match l {
            Level::Trace => LevelFilter::TRACE,
            Level::Debug => LevelFilter::DEBUG,
            Level::Info => LevelFilter::INFO,
            Level::Warn => LevelFilter::WARN,
            Level::Error => LevelFilter::ERROR,
        }
    }
}
