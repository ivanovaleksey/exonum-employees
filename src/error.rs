use exonum::crypto::PublicKey;
use toml::de::Error as TomlError;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Not a superuser public key: {:?}", _0)]
    BadSuperuserPublicKey(PublicKey),

    #[fail(display = "Failed to parse config: {}", _0)]
    InvalidConfig(#[cause] TomlError),

    #[fail(display = "Config for `employees` service not found")]
    ConfigNotFound,
}
