use exonum::crypto::PublicKey;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Not a superuser public key: {:?}", _0)]
    BadSuperuserPublicKey(PublicKey),
}
