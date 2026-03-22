use core::marker::PhantomData;

/// A generic structure for representing instruction input with associated account, packed account, and packed data types.
#[derive(Debug)]
pub struct GenericInstructionInput<A, PA, PD>(PhantomData<(A, PA, PD)>);
