extern crate diesel;
extern crate exonum;
extern crate exonum_employees;
extern crate failure;
extern crate toml;

use exonum::node::{Node, NodeConfig};
use exonum::storage::MemoryDB;

use exonum_employees::config;
use exonum_employees::service::{self, EmployeeService};

use std::process;

macro_rules! die {
    ($err:ident) => {{
        println!("{}", $err);
        process::exit(1);
    }};
}

fn parse_node_config() -> Result<NodeConfig, failure::Error> {
    use std::fs::File;
    use std::io::Read;

    let mut s = String::new();
    let mut file = File::open("config.toml")?;
    file.read_to_string(&mut s)?;

    let node_config = toml::from_str(&s)?;
    Ok(node_config)
}

fn main() {
    exonum::helpers::init_logger().unwrap();

    match main_() {
        Ok(node_config) => {
            let node = Node::new(
                MemoryDB::new(),
                vec![Box::new(EmployeeService)],
                node_config,
            );

            println!("Ready...");
            node.run().unwrap();
        }
        Err(e) => die!(e),
    }
}

fn main_() -> Result<NodeConfig, failure::Error> {
    let node_config = parse_node_config()?;
    let service_config = node_config
        .services_configs
        .get("employees")
        .cloned()
        .ok_or(config::Error::NotFound)?;
    let service_config = service_config.try_into::<config::Config>()?;

    let key = service_config.superuser_public_key;
    exonum_employees::check_superuser_public_key(&key)?;
    service::set_superuser_public_key(key);

    Ok(node_config)
}
