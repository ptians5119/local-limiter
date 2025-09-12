use clap::Parser;
use anyhow::{Result, anyhow};
use std::{fs::File, io::Read};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {
    pub web: WebConfig,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WebConfig {
    pub addr: String,
}

pub async fn parse() -> Result<Config> {
    let args = super::arg::Args::parse();
    let mut file = File::open(args.config_path).map_err(|err| anyhow!(err))?;
    let mut buffer = String::new();
    let _ = file.read_to_string(&mut buffer).map_err(|err| anyhow!(err))?;
    let config: Config = serde_yml::from_str(buffer.as_str()).map_err(|err| anyhow!(err))?;
    println!("config: {:?}", &config);
    Ok(config)
}