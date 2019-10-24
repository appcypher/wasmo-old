#[derive(Debug)]
pub struct ParserError {
    message: &'static str,
    offset: Offset,
}

#[derive(Debug)]
pub enum Offset {
    Unknown,
    Number(usize)
}


impl ParserError {
    pub fn new(message: &'static str, offset: Offset ) -> Self {
        Self { message, offset }
    }
}

impl From<&'static str> for ParserError {
    fn from(message: &'static str) -> Self {
        Self {
            message,
            offset: Offset::Unknown,
        }
    }
}

pub type ParserResult<T> = Result<T, ParserError>;
