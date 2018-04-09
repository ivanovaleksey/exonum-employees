use exonum::api::Api;
use exonum::blockchain::{ApiContext, Service, Transaction, TransactionSet};
use exonum::crypto::{Hash, PublicKey};
use exonum::encoding;
use exonum::messages::RawTransaction;
use exonum::storage::Snapshot;
use iron::Handler;
use router::Router;

use api::EmployeeApi;
use schema::EmployeeSchema;
use transactions::EmployeeTransactions;

static mut SUPERUSER_PUBLIC_KEY: Option<PublicKey> = None;

pub const SERVICE_ID: u16 = 1;

pub struct EmployeeService;

impl Service for EmployeeService {
    fn service_id(&self) -> u16 {
        SERVICE_ID
    }

    fn service_name(&self) -> &'static str {
        "employees"
    }

    fn state_hash(&self, snapshot: &Snapshot) -> Vec<Hash> {
        let schema = EmployeeSchema::new(snapshot);
        schema.state_hash()
    }

    fn tx_from_raw(&self, raw: RawTransaction) -> Result<Box<Transaction>, encoding::Error> {
        let tx = EmployeeTransactions::tx_from_raw(raw)?;
        Ok(tx.into())
    }

    fn public_api_handler(&self, ctx: &ApiContext) -> Option<Box<Handler>> {
        let mut router = Router::new();
        let api = EmployeeApi::new(ctx.node_channel().clone(), ctx.blockchain().clone());
        api.wire(&mut router);
        Some(Box::new(router))
    }
}

pub fn get_superuser_public_key() -> PublicKey {
    unsafe { SUPERUSER_PUBLIC_KEY.expect("Superuser public key must be set") }
}

// pub fn set_superuser_public_key(key: PublicKey) {
//     unsafe {
//         SUPERUSER_PUBLIC_KEY = Some(key);
//     }
// }
