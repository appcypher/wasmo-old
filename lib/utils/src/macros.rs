#[macro_export]
macro_rules! debug {
    ($string:tt $(, $expr:expr)*) => {
        if cfg!(any(debug_assertions, feature="debug")) {
            println!(concat!($string, "\n") $(, $expr)*)
        }
    };
}

#[macro_export]
macro_rules! verbose {
    ($string:tt $(, $expr:expr)*) => {
        if cfg!(feature="verbose") {
            println!(concat!($string, "\n") $(, $expr)*)
        }
    };
}
