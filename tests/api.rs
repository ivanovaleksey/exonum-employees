extern crate exonum;
extern crate exonum_employees;
extern crate exonum_testkit;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_json;

use exonum::crypto::{CryptoHash, Hash, PublicKey, Signature};
use exonum::encoding::serialize::FromHex;

use exonum_testkit::{ApiKind, TestKit, TestKitApi, TestKitBuilder};

use exonum_employees::schema::{Employee, EmployeeId};
use exonum_employees::service::EmployeeService;
use exonum_employees::transactions as tx;

lazy_static! {
    static ref JOHN_PUBLIC_KEY: PublicKey = PublicKey::from_hex("8c76b159144a765cc986b731c040d718c853c5c27e32f21f6c666fc463b868fd").unwrap();
    static ref JOHN_UPDATE_SIG: Signature = Signature::from_hex("f3f65a2f7ddd806fb20e542750ac5102f423d9fe23f62afb8be3ed2691319922a243218ee83c9c57e6a300b508f58bfbb92b5859a9b56c88cc9ae45723ebd503").unwrap();
}

#[test]
fn test_create_employee() {
    let (mut testkit, api) = create_testkit();

    let tx = api.create_employee();
    testkit.create_block();
    api.assert_tx_status(&tx.hash(), &json!({ "type": "success" }));

    let employee = api.get_employee(tx.id());

    assert_eq!(employee.public_key(), tx.public_key());
    assert_eq!(employee.id(), tx.id());
    assert_eq!(employee.first_name(), tx.first_name());
    assert_eq!(employee.last_name(), tx.last_name());
    assert_eq!(employee.info(), tx.info());
}

#[test]
fn test_update_existing_employee() {
    let (mut testkit, api) = create_testkit();

    let tx = api.create_employee();
    testkit.create_block();
    api.assert_tx_status(&tx.hash(), &json!({ "type": "success" }));

    let tx_update = tx::Update::new_with_signature(
        &JOHN_PUBLIC_KEY,
        1,
        "John",
        "Doe Jr.",
        "Personal info [UPDATED]",
        &JOHN_UPDATE_SIG,
    );
    api.update_employee(&tx_update);
    testkit.create_block();
    api.assert_tx_status(&tx_update.hash(), &json!({ "type": "success" }));

    let employee = api.get_employee(tx.id());

    assert_eq!(employee.public_key(), tx.public_key());
    assert_eq!(employee.id(), tx.id());
    assert_eq!(employee.first_name(), tx_update.first_name());
    assert_eq!(employee.last_name(), tx_update.last_name());
    assert_eq!(employee.info(), tx_update.info());
}

#[test]
fn test_update_nonexisting_employee() {
    let (mut testkit, api) = create_testkit();

    let tx_update = tx::Update::new_with_signature(
        &JOHN_PUBLIC_KEY,
        1,
        "John",
        "Doe Jr.",
        "Personal info [UPDATED]",
        &JOHN_UPDATE_SIG,
    );
    api.update_employee(&tx_update);
    testkit.create_block();
    api.assert_tx_status(
        &tx_update.hash(),
        &json!({ "type": "error", "code": 1, "description": "Employee not found" }),
    );

    api.assert_no_employee(tx_update.id());
}

struct EmployeeApi {
    pub inner: TestKitApi,
}

impl EmployeeApi {
    fn create_employee(&self) -> tx::Create {
        let sig = Signature::from_hex("7afa2155084112eb6e95629bb5afbc09026c836661dd867fa90d20e175335653216b3274087a6e93ad625bffafd17d19bef60d541bb92a661254c057fae1ea06").unwrap();
        let tx = tx::Create::new_with_signature(
            &JOHN_PUBLIC_KEY,
            1,
            "John",
            "Doe",
            "Personal info",
            &sig,
        );

        let tx_info: serde_json::Value =
            self.inner
                .post(ApiKind::Service("employees"), "employees", &tx);
        assert_eq!(tx_info, json!({ "tx_hash": tx.hash() }));
        tx
    }

    fn update_employee(&self, tx: &tx::Update) {
        let tx_info: serde_json::Value =
            self.inner
                .post(ApiKind::Service("employees"), "employees", &tx);
        assert_eq!(tx_info, json!({ "tx_hash": tx.hash() }));
    }

    fn get_employee(&self, id: EmployeeId) -> Employee {
        self.inner
            .get(ApiKind::Service("employees"), &format!("employees/{}", id))
    }

    fn assert_tx_status(&self, tx_hash: &Hash, expected_status: &serde_json::Value) {
        let info: serde_json::Value = self.inner.get(
            ApiKind::Explorer,
            &format!("v1/transactions/{}", tx_hash.to_string()),
        );
        if let serde_json::Value::Object(mut info) = info {
            let tx_status = info.remove("status").unwrap();
            assert_eq!(tx_status, *expected_status);
        } else {
            panic!("Invalid transaction info format, object expected");
        }
    }

    fn assert_no_employee(&self, id: EmployeeId) {
        let err: String = self.inner
            .get_err(ApiKind::Service("employees"), &format!("employees/{}", id));
        assert_eq!(err, "Not found".to_owned());
    }
}

fn create_testkit() -> (TestKit, EmployeeApi) {
    let testkit = TestKitBuilder::validator()
        .with_service(EmployeeService)
        .create();
    let api = EmployeeApi {
        inner: testkit.api(),
    };
    (testkit, api)
}
