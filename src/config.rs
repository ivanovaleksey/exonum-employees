use exonum::crypto::PublicKey;
use toml::de::Error as TomlError;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub superuser_public_key: PublicKey,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Config for `employees` service not found")]
    NotFound,

    #[fail(display = "Failed to parse config: {}", _0)]
    Invalid(#[cause] TomlError),
}
