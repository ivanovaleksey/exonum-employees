use exonum::blockchain::ExecutionError;

#[derive(Debug, Fail)]
#[repr(u8)]
pub enum Error {
    #[fail(display = "Bad public key")]
    BadPublicKey = 0,

    #[fail(display = "Employee not found")]
    EmployeeNotFound = 1,

    #[fail(display = "Employee already exists")]
    EmployeeAlreadyExists = 2,
}

impl From<Error> for ExecutionError {
    fn from(e: Error) -> Self {
        let description = format!("{}", e);
        let code = e as u8;
        ExecutionError::with_description(code, description)
    }
}
