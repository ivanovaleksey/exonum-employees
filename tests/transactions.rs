extern crate exonum;
extern crate exonum_employees;
extern crate exonum_testkit;
#[macro_use]
extern crate lazy_static;

use exonum::blockchain::Transaction;
use exonum::crypto::{self, PublicKey, Signature};
use exonum::encoding::serialize::FromHex;

use exonum_testkit::{TestKit, TestKitBuilder};

use exonum_employees::schema::{Employee, EmployeeId, EmployeeSchema};
use exonum_employees::service::EmployeeService;
use exonum_employees::transactions as tx;

lazy_static! {
    static ref JOHN_PUBLIC_KEY: PublicKey = PublicKey::from_hex("8c76b159144a765cc986b731c040d718c853c5c27e32f21f6c666fc463b868fd").unwrap();
    static ref JOHN_UPDATE_SIG: Signature = Signature::from_hex("f3f65a2f7ddd806fb20e542750ac5102f423d9fe23f62afb8be3ed2691319922a243218ee83c9c57e6a300b508f58bfbb92b5859a9b56c88cc9ae45723ebd503").unwrap();
    static ref JOHNNY_PUBLIC_KEY: PublicKey = PublicKey::from_hex("c8e91d252ca9454dddb68a19a034172720084f96ef1cea1fb1d804a5baf8f3bd").unwrap();
}

#[test]
fn test_create_employee_with_nonsu_key() {
    let (public_key, secret_key) = crypto::gen_keypair();
    let tx = tx::Create::new(&public_key, 1, "John", "Doe", "Personal info", &secret_key);

    assert!(!tx.verify());
}

#[test]
fn test_create_employee_with_su_key() {
    let mut testkit = init_testkit();

    let tx = create_employee_1(&mut testkit);

    let employee = get_employee(&testkit, 1);

    assert_eq!(employee.public_key(), tx.public_key());
    assert_eq!(employee.id(), tx.id());
    assert_eq!(employee.first_name(), tx.first_name());
    assert_eq!(employee.last_name(), tx.last_name());
    assert_eq!(employee.info(), tx.info());
}

#[test]
fn test_update_employee_with_nonsu_key() {
    let tx_update = tx::Update::new_with_signature(
        &JOHN_PUBLIC_KEY,
        1,
        "John",
        "Doe Jr.",
        "Personal info [UPDATED]",
        &JOHN_UPDATE_SIG,
    );

    assert!(tx_update.verify());
}

#[test]
fn test_update_employee_with_su_key() {
    let sig = Signature::from_hex("1517e5a486e58837d1039847d0fa7286ed6df36be748f4ea54565e0da20546dc7c5c64b9eaccc815d351cb9f8d2d6df4ca33306dd41ff522afc5ee51ec72d404").unwrap();
    let tx_update = tx::Update::new_with_signature(
        &JOHNNY_PUBLIC_KEY,
        2,
        "Johnny",
        "Appleseed",
        "Personal info [UPDATED]",
        &sig,
    );

    assert!(tx_update.verify());
}

#[test]
fn test_update_existing_employee() {
    let mut testkit = init_testkit();

    let tx = create_employee_1(&mut testkit);

    let tx_update = tx::Update::new_with_signature(
        &JOHN_PUBLIC_KEY,
        1,
        "John",
        "Doe Jr.",
        "Personal info [UPDATED]",
        &JOHN_UPDATE_SIG,
    );
    testkit.create_block_with_transaction(tx_update.clone());

    let employee = get_employee(&testkit, 1);

    assert_eq!(employee.public_key(), tx.public_key());
    assert_eq!(employee.id(), tx.id());
    assert_eq!(employee.first_name(), tx_update.first_name());
    assert_eq!(employee.last_name(), tx_update.last_name());
    assert_eq!(employee.info(), tx_update.info());
}

#[test]
fn test_update_nonexisting_employee() {
    let mut testkit = init_testkit();

    let tx_update = tx::Update::new_with_signature(
        &JOHN_PUBLIC_KEY,
        1,
        "John",
        "Doe Jr.",
        "Personal info [UPDATED]",
        &JOHN_UPDATE_SIG,
    );
    testkit.create_block_with_transaction(tx_update);

    let employee = try_get_employee(&testkit, 1);
    assert!(employee.is_none());
}

fn init_testkit() -> TestKit {
    TestKitBuilder::validator()
        .with_service(EmployeeService)
        .create()
}

fn create_employee_1(testkit: &mut TestKit) -> tx::Create {
    let sig = Signature::from_hex("7afa2155084112eb6e95629bb5afbc09026c836661dd867fa90d20e175335653216b3274087a6e93ad625bffafd17d19bef60d541bb92a661254c057fae1ea06").unwrap();
    let tx =
        tx::Create::new_with_signature(&JOHN_PUBLIC_KEY, 1, "John", "Doe", "Personal info", &sig);
    testkit.create_block_with_transaction(tx.clone());
    tx
}

fn get_employee(testkit: &TestKit, id: EmployeeId) -> Employee {
    try_get_employee(testkit, id).expect("No employee")
}

fn try_get_employee(testkit: &TestKit, id: EmployeeId) -> Option<Employee> {
    let snapshot = testkit.snapshot();
    EmployeeSchema::new(snapshot).employee(id)
}
