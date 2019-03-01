#[macro_export]
macro_rules! vararg_impl {
    ($( $ty:ident ),*) => {
        impl<'a, $( $ty: Type, )*> Func<'a, ( $( $ty, )* )> {
            pub fn call(&self, $( _: $ty, )*) {
                let func = unsafe { std::mem::transmute::<usize, fn ( $( $ty ),* )>(self.addr) };
                println!("call({:?}): addr({:p})", concat!($( stringify!($ty), )*), &func);
            }
        }
    };
}

#[macro_export]
macro_rules! recurse_vararg_impl {
    ($ty:ident) => {
        vararg_impl!($ty);
    };
    ($ty0:ident, $( $ty:ident ),+) => {
        vararg_impl!($ty0, $( $ty ),+);
        rec_vararg_impl!($( $ty ),+);
    };
}
