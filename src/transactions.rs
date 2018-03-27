use exonum::blockchain::{ExecutionResult, Transaction};
use exonum::crypto::PublicKey;
use exonum::messages::Message;
use exonum::storage::Fork;

use error::Error;
use schema::{Employee, EmployeeId, EmployeeSchema};
use service::{SERVICE_ID, SUPERUSER_PUBLIC_KEY};

transactions! {
    pub EmployeeTransactions {
        const SERVICE_ID = SERVICE_ID;

        struct Create {
            public_key: &PublicKey,
            id: EmployeeId,
            first_name: &str,
            last_name: &str,
            info: &str,
        }

        struct Update {
            public_key: &PublicKey,
            id: EmployeeId,
            first_name: &str,
            last_name: &str,
            info: &str,
        }
    }
}

impl Transaction for Create {
    fn verify(&self) -> bool {
        self.verify_signature(&SUPERUSER_PUBLIC_KEY)
    }

    fn execute(&self, view: &mut Fork) -> ExecutionResult {
        let mut schema = EmployeeSchema::new(view);

        match schema.employee(self.id()) {
            Some(_) => unreachable!(),
            None => {
                let employee = Employee::new(
                    self.public_key(),
                    self.id(),
                    self.first_name(),
                    self.last_name(),
                    self.info(),
                );
                println!("Tx Create: {:?}", employee);

                schema.employees_mut().put(&self.id(), employee);
                Ok(())
            }
        }
    }
}

impl Transaction for Update {
    fn verify(&self) -> bool {
        self.verify_signature(&SUPERUSER_PUBLIC_KEY) || self.verify_signature(self.public_key())
    }

    fn execute(&self, view: &mut Fork) -> ExecutionResult {
        let mut schema = EmployeeSchema::new(view);

        match schema.employee(self.id()) {
            Some(employee) => {
                if employee.public_key() != self.public_key() {
                    Err(Error::BadPublicKey)?
                }

                let employee = Employee::new(
                    employee.public_key(),
                    employee.id(),
                    self.first_name(),
                    self.last_name(),
                    self.info(),
                );
                println!("Tx Update: {:?}", employee);

                schema.employees_mut().put(&employee.id(), employee);
                Ok(())
            }
            None => Err(Error::EmployeeNotFound)?,
        }
    }
}
