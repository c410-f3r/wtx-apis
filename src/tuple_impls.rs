macro_rules! impl_tuples {
  ($( [$total:literal, $($T:ident($N:tt))*] )*) => {
    #[cfg(feature = "ethereum")]
    #[expect(non_snake_case, reason = "blame rustc devs")]
    mod ethereum {
      use crate::blockchain::ethereum::{SolToken, SolTokenSeq, SolTy, Word};
      use crate::blockchain::ethereum::Decoder;
      use crate::blockchain::ethereum::Encoder;
      use wtx::collection::Vector;
      use crate::blockchain::ethereum::misc::keccak256;

      $(
        impl<'de, $($T,)*> SolToken<'de> for ($( $T, )*)
        where
          $($T: SolToken<'de>,)*
        {
          const IS_DYN_TOKEN: bool = $( <$T as SolToken>::IS_DYN_TOKEN || )* false;

          fn decode_from(dec: &mut Decoder<'de>) -> crate::Result<Self> {
            if Self::IS_DYN_TOKEN {
              Self::decode_sequence(&mut dec.take_tail()?)
            } else {
              Self::decode_sequence(dec)
            }
          }

          fn head_push(&self, enc: &mut Encoder) -> crate::Result<()> {
            if Self::IS_DYN_TOKEN {
              enc.push_last_tail_idx()?;
            } else {
              let ($($T,)*) = self;
              $( $T.head_push(enc)?; )*
            }
            Ok(())
          }

          fn head_words(&self) -> usize {
            if Self::IS_DYN_TOKEN {
              1
            } else {
              let ($($T,)*) = self;
              0 $( + $T.total_words() )*
            }
          }

          fn tail_push(&self, enc: &mut Encoder) -> crate::Result<()> {
            if Self::IS_DYN_TOKEN {
              self.encode_sequence(enc)?;
            }
            Ok(())
          }

          fn tail_words(&self) -> usize {
            if Self::IS_DYN_TOKEN {
              let ($($T,)*) = self;
              0 $( + $T.total_words() )*
            } else {
              0
            }
          }
        }

        #[allow(non_snake_case, reason = "blame rustc devs")]
        impl<'de, $($T,)*> SolTokenSeq<'de> for ($( $T, )*)
        where
          $($T: SolToken<'de>,)*
        {
          #[inline]
          fn decode_sequence(_dec: &mut Decoder<'de>) -> crate::Result<Self> {
            Ok(($(
              match <$T as SolToken<'_>>::decode_from(&mut *_dec) {
                Ok(el) => el,
                Err(err) => return Err(err),
              },
            )*))
          }

          fn encode_sequence(&self, enc: &mut Encoder) -> crate::Result<()> {
            let ($($T,)*) = self;
            enc.push_tail_idx_by_words(0usize $( .wrapping_add($T.head_words()) )*)?;
            $(
              $T.head_push(enc)?;
              enc.bump_tail_idx($T.tail_words());
            )*
            $( $T.tail_push(enc)?; )*
            let _opt = enc.pop_tail_idx();
            Ok(())
          }
        }

        impl<'de, $($T: SolTy<'de>,)*> SolTy<'de> for ($($T,)*) {
          #[allow(unused_labels, reason = "0-arity tuple")]
          const ENCODED_SIZE: Option<usize> = 'block: {
            Some(
              0 $(+ {
                match <$T as SolTy>::ENCODED_SIZE {
                  Some(size) => size,
                  None => break 'block None,
                }
              })*
            )
          };
          #[allow(unused_labels, reason = "0-arity tuple")]
          const PACKED_ENCODED_SIZE: Option<usize> = 'block: {
            Some(
              0 $(+ {
                match <$T as SolTy>::PACKED_ENCODED_SIZE {
                  Some(size) => size,
                  None => break 'block None,
                }
              })*
            )
          };
          const SOL_NAME: &'static str = StaticString::new()
            .write_str("(")
            $(
              .write_str($T::SOL_NAME)
              .write_str(",")
            )*
            .pop()
            .unwrap()
            .write_str(")")
            .as_str();

          type DeToken<'any> = ($( $T::DeToken<'any>, )*);
          type Token<'any> = ($( $T::Token<'any>, )*);

          fn abi_encode_packed(&self, _buffer: &mut Vector<u8>) -> crate::Result<()> {
            let ($($T,)*) = self;
            $( $T.abi_encode_packed(_buffer)?; )*
            Ok(())
          }

          fn abi_encoded_size(&self) -> usize {
            if let Some(size) = <($($T,)*) as SolTy<'de>>::ENCODED_SIZE {
              return size
            }

            let ($($T,)*) = self;
            let sum = 0 $( + $T.abi_encoded_size() )*;
            if <($($T,)*) as SolTy<'de>>::IS_DYN_TY {
              32 + sum
            } else {
              sum
            }
          }

          fn abi_packed_encoded_size(&self) -> usize {
            let ($($T,)*) = self;
            0 $(+ $T.abi_packed_encoded_size())*
          }

          fn detokenize(token: Self::Token<'de>) -> crate::Result<Self::DeToken<'de>> {
            let ($($T,)*) = token;
            Ok(($( <$T as SolTy<'de>>::detokenize($T)?, )*))
          }

          fn eip712_data_word(&self, _buffer: &mut Vector<u8>) -> crate::Result<Word> {
            let ($($T,)*) = self;
            let encoding: [[u8; 32]; $total] = [$( <$T as SolTy>::eip712_data_word($T, _buffer)?.0, )*];
            // SAFETY: Flattening [[u8; 32]; $count] to [u8; $count * 32] is valid
            let encoding: &[u8] = unsafe { core::slice::from_raw_parts(encoding.as_ptr().cast(), $total * 32) };
            Ok(Word(keccak256([encoding])))
          }

          fn tokenize(&self) -> crate::Result<Self::Token<'_>> {
            let ($($T,)*) = self;
            Ok(($($T.tokenize()?,)*))
          }

          fn valid_token(token: &Self::Token<'_>) -> bool {
            let ($($T,)*) = token;
            $(<$T as SolTy>::valid_token($T) &&)* true
          }
        }
      )*

      // TODO(upstream): Use ArrayStringU8
      #[must_use]
      struct StaticString {
        _buffer: [u8; 256],
        len: usize,
      }

      impl StaticString {
        const fn new() -> Self {
          Self { _buffer: [0; 256], len: 0 }
        }

        const fn as_str(&self) -> &str {
          // SAFETY: `len` is within bounds
          let bytes = unsafe { core::slice::from_raw_parts(self._buffer.as_ptr(), self.len) };
          match core::str::from_utf8(bytes) {
            Ok(elem) => elem,
            Err(_) => panic!(),
          }
        }

        const fn pop(mut self) -> Option<Self> {
          let len = match self.len.checked_sub(1) {
            None => return None,
            Some(elem) => elem,
          };
          self.len = len;
          Some(self)
        }

        const fn write_str(mut self, str: &str) -> Self {
          let bytes = str.as_bytes();
          let mut idx = 0;
          while idx < bytes.len() {
            self._buffer[self.len.wrapping_add(idx)] = bytes[idx];
            idx = idx.wrapping_add(1);
          }
          self.len = self.len.wrapping_add(bytes.len());
          self
        }
      }
    }
  }
}

impl_tuples! {
  [0,]
  [1, A(0)]
  [2, A(0) B(1)]
  [3, A(0) B(1) C(2)]
  [4, A(0) B(1) C(2) D(3)]
  [5, A(0) B(1) C(2) D(3) E(4)]
  [6, A(0) B(1) C(2) D(3) E(4) F(5)]
}
