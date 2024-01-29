use thiserror::Error;


#[derive(Debug, Error)]
pub enum DumpError {

}


pub trait Dumpable {

    fn dump(&mut self) -> Result<(), DumpError>;
} 