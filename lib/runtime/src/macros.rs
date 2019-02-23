#[macro_export]
macro_rules! vararg_impl {
    ($( $ty:ident ),*) => {
        impl<'a, $( $ty: Type, )*> Func<'a, ( $( $ty, )* )> {
            pub fn call(&self, $( _: $ty, )*) {
                println!("Call({:?})", concat!($( stringify!($ty), )*));
            }
        }
    };
}

#[macro_export]
macro_rules! rec_vararg_impl {
    ($ty:ident) => {
        vararg_impl!($ty);
    };
    ($ty0:ident, $( $ty:ident ),+) => {
        vararg_impl!($ty0, $( $ty ),+);
        rec_vararg_impl!($( $ty ),+);
    };
}
