use thiserror::Error;


#[derive(Debug, Error)]
pub enum DumpError {

}


pub trait Dumpable {

    fn dump(&self) -> Result<(), DumpError>;
} 