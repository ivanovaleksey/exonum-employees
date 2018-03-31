# Exonum Employees

## Installation

Application uses PostgreSQL database.
In order to prepare a database one can use [Diesel CLI](https://github.com/diesel-rs/diesel/tree/master/diesel_cli) tool

```
export DATABASE_URL=postgres://localhost/exonum_employees
diesel database reset
```

## Usage

### Generate transactions

There is a simple binary which provides generation of keypairs and signatures.  
One can run it by executing

```
cargo run --bin gen
```

Current application and scripts are set up to work with the following keys and signatures

```
Superuser public key: 8d91b28b9ef9e8745d04fe114657dc95ee41ef34502a51dd7f3defc117ed95e5

Transaction: Create { public_key: PublicKey(8C76B159), id: 1, first_name: "John", last_name: "Doe", info: "Personal info" }
Public key: 8c76b159144a765cc986b731c040d718c853c5c27e32f21f6c666fc463b868fd
Signature: 7afa2155084112eb6e95629bb5afbc09026c836661dd867fa90d20e175335653216b3274087a6e93ad625bffafd17d19bef60d541bb92a661254c057fae1ea06

Transaction: Update { public_key: PublicKey(8C76B159), id: 1, first_name: "John", last_name: "Doe Jr.", info: "Personal info [UPDATED]" }
Public key: 8c76b159144a765cc986b731c040d718c853c5c27e32f21f6c666fc463b868fd
Signature: f3f65a2f7ddd806fb20e542750ac5102f423d9fe23f62afb8be3ed2691319922a243218ee83c9c57e6a300b508f58bfbb92b5859a9b56c88cc9ae45723ebd503

Transaction: Create { public_key: PublicKey(C8E91D25), id: 2, first_name: "Johnny", last_name: "Appleseed", info: "Personal info" }
Public key: c8e91d252ca9454dddb68a19a034172720084f96ef1cea1fb1d804a5baf8f3bd
Signature: d56dd76b4627055385621a1972ece457a4ef7cc31b3f0ade5e70a0b9cdcca8f939099f699c9691fa6e09060a3a7b7dd5ad1086a474d5bc6d4e83621ba6b8d20f

Transaction: Update { public_key: PublicKey(C8E91D25), id: 2, first_name: "Johnny", last_name: "Appleseed", info: "Personal info [UPDATED]" }
Public key: c8e91d252ca9454dddb68a19a034172720084f96ef1cea1fb1d804a5baf8f3bd
Signature: 1517e5a486e58837d1039847d0fa7286ed6df36be748f4ea54565e0da20546dc7c5c64b9eaccc815d351cb9f8d2d6df4ca33306dd41ff522afc5ee51ec72d404
```

### Manage superuser key

Before running the application superuser public key must be set in `config.toml`. 
Provided key will be stored in database if there are no keys at the moment. If there are any keys provided key will be compared with those in database. If the key is present in database it will be set as superuser public key for `EmployeeService`, otherwise application will not start.

There is CLI tool called `keychain` which helps you to manage (add, list, remove) available superuser keys.

```
cargo run --bin keychain

# Add a key
cargo run --bin keychain add 8d91b2...

# List all keys
cargo run --bin keychain ls

# Remove a key
cargo run --bin keychain rm 8d91b2...
```

### Run blockchain

In order to run a blockchain execute

```
cargo run --bin main
```

`scripts/` directory contains transaction files and helper scripts to interact with the blockchain.  
Use `curl` command to send transactions to the blockchain

```
curl -H "Content-Type: application/json" -X POST -d @create-employee-1.json  \
    http://127.0.0.1:8000/api/services/employees/employees

curl -H "Content-Type: application/json" -X POST -d @create-employee-2.json  \
    http://127.0.0.1:8000/api/services/employees/employees

curl -H "Content-Type: application/json" -X POST -d @update-employee-1.json  \
    http://127.0.0.1:8000/api/services/employees/employees

curl -H "Content-Type: application/json" -X POST -d @update-employee-2.json  \
    http://127.0.0.1:8000/api/services/employees/employees
```

There is a script which helps to get information about employees

```
# Display info for all employees
./employees.sh --all

# Display info for employee with id 1
./employees.sh --key 1

# Display info for employee blocks
./employees.sh --blocks 1
```
