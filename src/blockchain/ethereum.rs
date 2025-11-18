//! Structures that allow the interaction with the Ethereum network
//!
//! Many elements were inspired from the `alloy` project.

mod address;
mod decoder;
mod eip712;
mod encoder;
pub(crate) mod misc;
mod packed_seq;
mod sol_int;
mod sol_struct;
mod sol_token;
mod sol_token_seq;
mod sol_ty;
mod word;

pub use address::Address;
pub use decoder::Decoder;
pub use eip712::{Eip712, Eip712Domain};
pub use encoder::Encoder;
pub use packed_seq::PackedSeq;
pub use sol_int::SolInt;
pub use sol_struct::SolStruct;
pub use sol_token::SolToken;
pub use sol_token_seq::SolTokenSeq;
pub use sol_ty::SolTy;
pub use word::Word;
