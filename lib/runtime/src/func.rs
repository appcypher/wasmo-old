
use std::marker::PhantomData;

///
pub struct Func<'a, Params = ()> {
    addr: *const (),
    phantom: PhantomData<&'a Params>,
}
