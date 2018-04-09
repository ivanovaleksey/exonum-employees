use exonum::crypto::PublicKey;
use toml::de::Error as TomlError;

lazy_static! {
    pub static ref CONFIG: RwLock<Config> = RwLock::new(Config::default());
}

// pub fn init() -> Result<(), failure::Error> {
//     let mut config = CONFIG.write().unwrap();

//     let mut c = Config::new();
//     c.merge(File::with_name("Settings.toml"))?;
//     *settings = c.try_into::<Settings>()?;

//     Ok(())
// }

#[derive(Debug, Deserialize)]
pub struct Config {
    pub superuser_public_key: PublicKey,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            superuser_public_key: PublicKey::default(),
        }
    }
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Config for `employees` service not found")]
    NotFound,

    #[fail(display = "Failed to parse config: {}", _0)]
    Invalid(#[cause] TomlError),
}
