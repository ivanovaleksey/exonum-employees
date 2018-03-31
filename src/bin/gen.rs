extern crate exonum;
extern crate exonum_employees;
#[macro_use]
extern crate quicli;

use exonum::crypto;
use exonum::messages::Message;
use exonum_employees::transactions;
use quicli::prelude::*;

/// Generate signed transactions
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "verbosity", short = "v", parse(from_occurrences))]
    verbosity: u8,
}

main!(|args: Cli, log_level: verbosity| {
    let (su_public_key, su_secret_key) = crypto::gen_keypair();
    println!("Superuser public key: {}", su_public_key.to_hex());

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

    // Employee #1 would be updated with it's own public key
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

    let (public_key, _) = crypto::gen_keypair();
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

    // Employee #2 would be updated with superuser public key
    let tx = transactions::Update::new(
        &public_key,
        2,
        "Johnny",
        "Appleseed",
        "Personal info [UPDATED]",
        &su_secret_key,
    );
    println!("Transaction: {:?}", tx);
    println!("Public key: {}", tx.public_key().to_hex());
    println!("Signature: {}\n", tx.raw().signature().to_hex());
});
