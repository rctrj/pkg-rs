use std::str::FromStr;

use config::{Config, Environment, File};
use dotenv::dotenv;
use serde::Deserialize;
use strum::{self, Display, EnumString};

use crate::errors::Result;

#[derive(Copy, Clone, Debug, Display, Eq, PartialEq, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Env {
    Test,
    Dev,
    Stage,
    Prod,
}

pub fn curr_env() -> Env {
    let env_str = std::env::var("ENV").expect("env not set");
    Env::from_str(env_str.as_str()).unwrap_or_else(|_| panic!("unrecognized env: {env_str}"))
}

pub fn is_local_env() -> bool {
    if let Ok(value) = std::env::var("is_local_env") {
        return value.to_lowercase() == "true";
    }

    false
}

pub fn parse_or_panic<'a, T: Deserialize<'a>>(dir: &str) -> T {
    match parse(dir) {
        Ok(out) => out,
        Err(err) => panic!("{}", err),
    }
}

pub fn parse<'a, T: Deserialize<'a>>(dir: &str) -> Result<T> {
    dotenv().ok();

    let env = curr_env();
    let dir = resolve_path(dir);

    let file_name = format!("{dir}/{env}.yml");
    let env_conf = Environment::default().separator("_");

    let config = Config::builder()
        .add_source(File::with_name(&file_name))
        .add_source(env_conf)
        .build()?;

    Ok(config.try_deserialize()?)
}

fn resolve_path(dir: &str) -> &str {
    let mut dir = dir.trim();

    if dir.ends_with('/') || dir.ends_with('\\') {
        let mut chars = dir.chars();

        chars.next_back();
        dir = chars.as_str();
    }

    dir
}
