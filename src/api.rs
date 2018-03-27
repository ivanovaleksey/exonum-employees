use bodyparser;
use exonum::api::{Api, ApiError};
use exonum::blockchain::{Blockchain, Transaction};
use exonum::crypto::Hash;
use exonum::node::{ApiSender, TransactionSend};
use iron::headers::ContentType;
use iron::modifiers::Header;
use iron::prelude::*;
use iron::status::Status;
use router::Router;
use serde_json;

use schema::EmployeeSchema;
use transactions::EmployeeTransactions;

#[derive(Clone)]
pub struct EmployeeApi {
    channel: ApiSender,
    blockchain: Blockchain,
}

impl EmployeeApi {
    pub fn new(channel: ApiSender, blockchain: Blockchain) -> EmployeeApi {
        EmployeeApi {
            channel,
            blockchain,
        }
    }

    fn post_transaction(&self, req: &mut Request) -> IronResult<Response> {
        match req.get::<bodyparser::Struct<EmployeeTransactions>>() {
            Ok(Some(transaction)) => {
                let transaction: Box<Transaction> = transaction.into();
                let tx_hash = transaction.hash();
                self.channel.send(transaction).map_err(ApiError::from)?;

                let resp = TransactionResponse { tx_hash };
                self.ok_response(&serde_json::to_value(resp).unwrap())
            }
            Ok(None) => unimplemented!(),
            Err(e) => Err(ApiError::BadRequest(e.to_string()))?,
        }
    }

    fn get_employees(&self, _req: &Request) -> IronResult<Response> {
        let snapshot = self.blockchain.snapshot();
        let schema = EmployeeSchema::new(snapshot);

        let employees = schema.employees();
        let employees = employees.values().collect::<Vec<_>>();

        self.ok_response(&serde_json::to_value(&employees).unwrap())
    }

    fn get_employee(&self, req: &Request) -> IronResult<Response> {
        let path = req.url.path();
        let employee_id = path.last().unwrap();
        let employee_id = employee_id.parse::<u16>().map_err(|e| {
            IronError::new(
                e,
                (
                    Status::BadRequest,
                    Header(ContentType::json()),
                    "\"Invalid id\"",
                ),
            )
        })?;

        let snapshot = self.blockchain.snapshot();
        let schema = EmployeeSchema::new(snapshot);

        match schema.employee(employee_id) {
            Some(employee) => self.ok_response(&serde_json::to_value(employee).unwrap()),
            None => self.not_found_response(&serde_json::to_value("Not found").unwrap()),
        }
    }
}

impl Api for EmployeeApi {
    fn wire(&self, router: &mut Router) {
        let self_ = self.clone();
        let post_create_employee = move |req: &mut Request| self_.post_transaction(req);

        let self_ = self.clone();
        let get_employees = move |req: &mut Request| self_.get_employees(req);

        let self_ = self.clone();
        let get_employee = move |req: &mut Request| self_.get_employee(req);

        router.post("/employees", post_create_employee, "post_create_employee");
        router.get("/employees", get_employees, "get_employees");
        router.get("/employees/:id", get_employee, "get_employee");
    }
}

#[derive(Debug, Serialize)]
struct TransactionResponse {
    tx_hash: Hash,
}
