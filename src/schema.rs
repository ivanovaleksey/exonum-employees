use exonum::crypto::PublicKey;
use exonum::storage::map_index::MapIndex;
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

    pub fn employees(&self) -> MapIndex<&Snapshot, EmployeeId, Employee> {
        MapIndex::new(MAP_NAME, self.view.as_ref())
    }

    pub fn employee(&self, id: EmployeeId) -> Option<Employee> {
        self.employees().get(&id)
    }
}

impl<'a> EmployeeSchema<&'a mut Fork> {
    pub fn employees_mut(&mut self) -> MapIndex<&mut Fork, EmployeeId, Employee> {
        MapIndex::new(MAP_NAME, self.view)
    }
}
