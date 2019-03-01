
use std::marker::PhantomData;

use crate::macros::recurse_vararg_impl;

use crate::types::Type;

///
pub struct Func<'a, Params = ()> {
    addr: *const (),
    phantom: PhantomData<&'a Params>,
}

impl<'a, Params> Func<'a, Params> {
    pub fn from_ptr(addr: *const ()) -> Func<'a, Params> {
        Self { addr, phantom: PhantomData }
    }
}

///
recurse_vararg_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);

///
trait Type {}
impl Type for i32 {}
impl Type for i64 {}
impl Type for f32 {}
impl Type for f64 {}
