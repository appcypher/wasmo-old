#[macro_export]
macro_rules! debug {
    ($string:tt $(, $expr:expr)*) => {
        if cfg!(any(debug_assertions, feature="debug")) {
            println!(concat!("\nwasmlite::", $string) $(, $expr)*)
        }
    };
}
