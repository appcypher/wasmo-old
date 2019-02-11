use crate::kinds::ErrorKind;

#[derive(Debug, Clone, PartialEq)]
pub struct ParserError {
    pub kind: ErrorKind,
    pub cursor: usize,
}
