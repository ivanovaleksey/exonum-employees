use exonum::crypto::PublicKey;
use failure;
use toml;

use std::fs::File;
use std::io::Read;

use error::Error;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub superuser_public_key: PublicKey,
}

pub fn parse() -> Result<Config, failure::Error> {
    let mut s = String::new();

    let mut file = File::open("config.toml")?;
    file.read_to_string(&mut s)?;
    let conf = toml::from_str(&s).map_err(Error::InvalidConfig)?;

    Ok(conf)
}
