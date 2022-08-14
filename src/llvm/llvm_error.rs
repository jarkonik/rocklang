use core::fmt;
use std::error::Error;

#[derive(Clone, Debug)]
pub struct LLVMError {}

impl fmt::Display for LLVMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LLVMError")
    }
}

impl Error for LLVMError {}
