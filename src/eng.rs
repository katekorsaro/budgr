pub mod list;

#[derive(PartialEq,Debug)]
pub enum EngErr {
    DataFileNotFound,
    DataFileNotReadable,
    WhileReadingFile,
}
