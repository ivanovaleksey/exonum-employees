extern crate diesel;
extern crate exonum;
extern crate exonum_employees;
extern crate failure;

use diesel::prelude::*;
use exonum::blockchain::{GenesisConfig, ValidatorKeys};
use exonum::crypto::PublicKey;
use exonum::node::{Node, NodeApiConfig, NodeConfig};
use exonum::storage::MemoryDB;

use exonum_employees::config;
use exonum_employees::db_schema::superuser_keys;
use exonum_employees::error::Error;
use exonum_employees::service::{self, EmployeeService};
use exonum_employees::superuser_key::NewSuperuserKey;

use std::process;

fn node_config() -> NodeConfig {
    let (consensus_public_key, consensus_secret_key) = exonum::crypto::gen_keypair();
    let (service_public_key, service_secret_key) = exonum::crypto::gen_keypair();

    let validator_keys = ValidatorKeys {
        consensus_key: consensus_public_key,
        service_key: service_public_key,
    };
    let genesis = GenesisConfig::new(vec![validator_keys].into_iter());

    let api_address = "0.0.0.0:8000".parse().unwrap();
    let api_cfg = NodeApiConfig {
        public_api_address: Some(api_address),
        ..Default::default()
    };

    let peer_address = "0.0.0.0:2000".parse().unwrap();

    NodeConfig {
        listen_address: peer_address,
        peers: vec![],
        service_public_key,
        service_secret_key,
        consensus_public_key,
        consensus_secret_key,
        genesis,
        external_address: None,
        network: Default::default(),
        whitelist: Default::default(),
        api: api_cfg,
        mempool: Default::default(),
        services_configs: Default::default(),
    }
}

fn main() {
    exonum::helpers::init_logger().unwrap();

    match config::parse() {
        Ok(conf) => {
            let key = conf.superuser_public_key;
            match check_superuser_public_key(&key) {
                Ok(_) => service::set_superuser_public_key(key),
                Err(e) => {
                    println!("{}", e);
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    }

    let node = Node::new(
        MemoryDB::new(),
        vec![Box::new(EmployeeService)],
        node_config(),
    );

    println!("Ready...");
    node.run().unwrap();
}

fn check_superuser_public_key(public_key: &PublicKey) -> Result<(), failure::Error> {
    let conn = exonum_employees::establish_connection();

    let count = superuser_keys::table.count().get_result::<i64>(&conn)?;
    if count == 0 {
        let new_key = NewSuperuserKey {
            public_key: public_key.to_hex(),
        };
        diesel::insert_into(superuser_keys::table)
            .values(&new_key)
            .execute(&conn)?;
        return Ok(());
    }

    let key_exists = diesel::select(diesel::dsl::exists(
        superuser_keys::table.filter(superuser_keys::public_key.eq(public_key.to_hex())),
    )).get_result(&conn)?;

    if key_exists {
        return Ok(());
    } else {
        return Err(Error::BadSuperuserPublicKey(*public_key))?;
    }
}
