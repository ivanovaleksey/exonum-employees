use exonum::crypto::PublicKey;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub superuser_public_key: PublicKey,
}
