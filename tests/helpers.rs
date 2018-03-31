extern crate exonum;
extern crate exonum_employees;

pub fn set_superuser_public_key() {
    use exonum::crypto::PublicKey;
    use exonum::encoding::serialize::FromHex;
    use exonum_employees::service;

    let key = PublicKey::from_hex(
        "8d91b28b9ef9e8745d04fe114657dc95ee41ef34502a51dd7f3defc117ed95e5",
    ).expect("Failed to build superuser public key");

    service::set_superuser_public_key(key);
}
