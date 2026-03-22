use crate::blockchain::solana::program::{U32LE, U64LE, compute_budget::data::Instruction};

create_and_impl_build_ix_input!(SetComputeUnitLimit, (), U32LE, Instruction, |wrapper| {
  Instruction::SetComputeUnitLimit(wrapper.elem)
});

create_and_impl_build_ix_input!(SetComputeUnitPrice, (), U64LE, Instruction, |wrapper| {
  Instruction::SetComputeUnitPrice(wrapper.elem)
});
