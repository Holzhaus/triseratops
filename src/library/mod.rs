//! Parsers for the Serato library database and crates
pub mod database;
pub(self) mod parser;

pub use parser::{Library, Track};
