
use std::marker::PhantomData;

///
#[derive(Debug)]
pub struct Func<'a, Params = ()> {
    addr: *const (),
    phantom: PhantomData<&'a Params>,
}

// TODO: Inline all .call methods.
