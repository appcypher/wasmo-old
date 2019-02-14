#[macro_export]
macro_rules! get_value {
    ($func:expr, $cursor:ident, $error0:ident, $error1:ident) => {
        match $func {
            Ok(value) => value,
            Err(error) => {
                if error == ErrorKind::BufferEndReached {
                    return Err(ParserError {
                        kind: ErrorKind::$error0,
                        cursor: $cursor,
                    });
                } else {
                    return Err(ParserError {
                        kind: ErrorKind::$error1,
                        cursor: $cursor,
                    });
                }
            }
        }
    };
}

#[macro_export]
macro_rules! get_end_byte {
    ($func:expr, $cursor:ident, $error0:ident, $error1:ident) => {
        match $func {
            Ok(value) => {
                // Check if end byte is correct.
                if value != 0x0b {
                    return Err(ParserError {
                        kind: ErrorKind::MalformedEndByteInFunctionBody,
                        cursor: $cursor,
                    });
                }
                value
            },
            Err(error) => {
                if error == ErrorKind::BufferEndReached {
                    return Err(ParserError {
                        kind: ErrorKind::$error0,
                        cursor: $cursor,
                    });
                } else {
                    return Err(ParserError {
                        kind: ErrorKind::$error1,
                        cursor: $cursor,
                    });
                }
            }
        }
    };
}

macro_rules! llvm {
    ($expr:expr) => {
        if cfg!(llvm) {
            $expr
        }
    };
}
