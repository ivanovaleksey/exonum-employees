extern crate exonum;
extern crate exonum_employees;

use exonum::crypto;
use exonum::messages::Message;
use exonum_employees::transactions;

fn main() {
    let (su_public_key, su_secret_key) = crypto::gen_keypair();
    println!("Superuser public key: {}", su_public_key.to_hex());
    println!("Superuser secret key: {}\n", su_secret_key.to_hex());

    let (public_key, secret_key) = crypto::gen_keypair();
    let tx = transactions::Create::new(
        &public_key,
        1,
        "John",
        "Doe",
        "Personal info",
        &su_secret_key,
    );
    println!("Transaction: {:?}", tx);
    println!("Public key: {}", tx.public_key().to_hex());
    println!("Signature: {}\n", tx.raw().signature().to_hex());

    let tx = transactions::Update::new(
        &public_key,
        1,
        "John",
        "Doe Jr.",
        "Personal info [UPDATED]",
        &secret_key,
    );
    println!("Transaction: {:?}", tx);
    println!("Public key: {}", tx.public_key().to_hex());
    println!("Signature: {}\n", tx.raw().signature().to_hex());

    let (public_key, _secret_key) = crypto::gen_keypair();
    let tx = transactions::Create::new(
        &public_key,
        2,
        "Johnny",
        "Appleseed",
        "Personal info",
        &su_secret_key,
    );
    println!("Transaction: {:?}", tx);
    println!("Public key: {}", tx.public_key().to_hex());
    println!("Signature: {}\n", tx.raw().signature().to_hex());
}
