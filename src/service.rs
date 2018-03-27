use exonum::api::Api;
use exonum::blockchain::{ApiContext, Service, Transaction, TransactionSet};
use exonum::crypto::{Hash, PublicKey};
use exonum::encoding;
use exonum::messages::RawTransaction;
use exonum::storage::Snapshot;
use iron::Handler;
use router::Router;

use api::EmployeeApi;
use transactions::EmployeeTransactions;

lazy_static! {
    pub static ref SUPERUSER_PUBLIC_KEY: PublicKey = {
        use exonum::encoding::serialize::FromHex;

        PublicKey::from_hex("1db774c9a2c12a16d5e3f3e72ade12e22ffd412f1e8ce7e0f28f931428e0ed95")
            .expect("Failed to build superuser public key")
    };
}

pub const SERVICE_ID: u16 = 1;

pub struct EmployeeService;

impl Service for EmployeeService {
    fn service_id(&self) -> u16 {
        SERVICE_ID
    }

    fn service_name(&self) -> &'static str {
        "employees"
    }

    fn state_hash(&self, _snapshot: &Snapshot) -> Vec<Hash> {
        vec![]
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
