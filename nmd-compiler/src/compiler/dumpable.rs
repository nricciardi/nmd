
pub enum DumpError {

}


pub trait Dumpable<T> {

    fn dump(output_path: T) -> Result<(), DumpError>;
} 