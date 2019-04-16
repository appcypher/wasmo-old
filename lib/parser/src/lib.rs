#[macro_use]
pub mod macros;
pub mod ir;
mod errors;
mod kinds;
mod parser;
mod stack;
mod validation;
mod operators;

pub use errors::ParserError;
pub use ir::{ExportDesc, Function, Global, Import, Type, ValueType, ImportDesc, Local, Memory, Module, Operator, Section, Table};
pub use kinds::{ErrorKind, SectionKind};
pub use parser::{Parser, ParserResult};
