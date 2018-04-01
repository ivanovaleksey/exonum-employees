use exonum::crypto::{CryptoHash, Hash, PublicKey};
use exonum::storage::proof_map_index::ProofMapIndex;
use exonum::storage::{Fork, Snapshot};

encoding_struct! {
    struct Employee {
        public_key: &PublicKey,
        id: EmployeeId,
        first_name: &str,
        last_name: &str,
        info: &str,
    }
}

pub type EmployeeId = u16;

pub struct EmployeeSchema<T> {
    view: T,
}

const MAP_NAME: &str = "employees.employees";

impl<T> EmployeeSchema<T>
where
    T: AsRef<Snapshot>,
{
    pub fn new(view: T) -> EmployeeSchema<T> {
        EmployeeSchema { view }
    }

    pub fn state_hash(&self) -> Vec<Hash> {
        vec![self.employees().root_hash()]
    }

    pub fn employees(&self) -> ProofMapIndex<&Snapshot, Hash, Employee> {
        ProofMapIndex::new(MAP_NAME, self.view.as_ref())
    }

    pub fn employee(&self, id: EmployeeId) -> Option<Employee> {
        self.employees().get(&id.hash())
    }
}

impl<'a> EmployeeSchema<&'a mut Fork> {
    pub fn put_employee(&mut self, id: &EmployeeId, employee: Employee) {
        let mut map = self.employees_mut();
        map.put(&id.hash(), employee);
    }

    fn employees_mut(&mut self) -> ProofMapIndex<&mut Fork, Hash, Employee> {
        ProofMapIndex::new(MAP_NAME, self.view)
    }
}
