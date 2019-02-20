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
