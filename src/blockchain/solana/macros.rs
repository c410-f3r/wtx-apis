macro_rules! commitment_doc {
  () => {
    "Additional set of optional parameters used by the corresponding request."
  };
}

macro_rules! create_account_struct {
  (
    $( #[$meta:meta] )*
    $vis:vis struct $name:ident<A> {
      $(
        $( #[$field_meta:meta] )*
        $field_vis:vis $field_ident:ident: $field_ty:ty
      ),* $(,)?
    }

    $($generic_ty_test:path)?
  ) => {
    /// Groups of accounts required to execute an instruction
    $( #[$meta] )*
    $vis struct $name<A> {
      $(
              /// Account
        $( #[$field_meta] )*
        $field_vis $field_ident: $field_ty
      ),*
    }

    #[allow(non_snake_case)]
    #[cfg(test)]
    #[test]
    fn has_correct_pack_state_account_values() {
      use crate::blockchain::solana::program::TestingInstances;

      crate::blockchain::solana::program::tests::test_account(
        $name:: $(<$generic_ty_test>::)? min_instance().unwrap(),
      );
      crate::blockchain::solana::program::tests::test_account(
        $name:: $(<$generic_ty_test>::)? variable_instance().unwrap(),
      );
    }
  };
}

macro_rules! create_data_enum {
  (
    $( #[$meta:meta] )*
    $vis:vis enum $name:ident {
      $(
        $( #[$variant_meta:meta] )*
        $variant_ident:ident$(($variant_ty:ty))?
      ),+ $(,)?
    }
  ) => {
    /// Conditional data stored in the blockchain
    $( #[$meta] )*
    $vis enum $name {
      $(
        /// Variant stored in the blockchain
        $( #[$variant_meta] )*
        $variant_ident$(($variant_ty))?
      ),+
    }

    #[allow(non_snake_case)]
    #[cfg(test)]
    #[test]
    fn _has_correct_pack_state_enum_values() {
      let mut greater_variant_len: usize = 0;
      let mut lesser_variant_len: usize = usize::MAX;

      $({
        crate::blockchain::solana::program::tests::test_data(
          $name::$variant_ident$((<$variant_ty as crate::blockchain::solana::program::TestingInstances>::min_instance().unwrap()))?,
          false,
          [&mut lesser_variant_len, &mut greater_variant_len]
        );
      })+
      assert_eq!(lesser_variant_len, $name::LEN_BOUNDS.min());

      $({
        crate::blockchain::solana::program::tests::test_data(
          $name::$variant_ident$((<$variant_ty as crate::blockchain::solana::program::TestingInstances>::variable_instance().unwrap()))?,
          false,
          [&mut lesser_variant_len, &mut greater_variant_len]
        );
      })+
      if let Some(max) = $name::LEN_BOUNDS.max() {
        assert_eq!(greater_variant_len, max);
      }
      assert_eq!(lesser_variant_len, $name::LEN_BOUNDS.min());
    }
  };
}

macro_rules! create_data_struct {
  (
    $( #[$meta:meta] )*
    $vis:vis struct $name:ident <$($generic:ident),*> {
      $(
        $( #[$field_meta:meta] )*
        $field_vis:vis $field_ident:ident: $field_ty:ty
      ),* $(,)?
    }

    $($generic_ty_test:path)?
  ) => {
    /// Data stored in the blockchain
    $( #[$meta] )*
    $vis struct $name <$($generic),*> {
      $(
        /// Field stored in the blockchain
        $( #[$field_meta] )*
        $field_vis $field_ident: $field_ty
      ),*
    }

    #[allow(non_snake_case)]
    #[cfg(test)]
    #[test]
    fn $name() {
      use crate::blockchain::solana::program::TestingInstances;

      crate::blockchain::solana::program::tests::test_data(
        $name:: $(<$generic_ty_test>::)? min_instance().unwrap(),
        true,
        [&mut 0, &mut 0]
      );
      crate::blockchain::solana::program::tests::test_data(
        $name:: $(<$generic_ty_test>::)? variable_instance().unwrap(),
        false,
        [&mut 0, &mut 0]
      );
    }
  };
}

macro_rules! create_and_impl_build_ix_input {
  (
    $ty_ident:ident,
    $pack_accounts:ty,
    $pack_data:ty,
    $transformed_pack_data:ty,
    $cb:expr $(,)?
  ) => {
    /// Instruction input with its associated data.
    pub type $ty_ident<A> =
      crate::blockchain::solana::program::GenericInstructionInput<A, $pack_accounts, $pack_data>;

    impl<A> crate::blockchain::solana::program::BuildIxInput<A> for $ty_ident<A>
    where
      A: crate::blockchain::solana::program::AccountAddress,
    {
      type PackAccounts = $pack_accounts;
      type PackData = $pack_data;
      type TransformedPackData = $transformed_pack_data;

      fn transform_pack_data(pack_data: Self::PackData) -> Self::TransformedPackData {
        let cb: fn(Self::PackData) -> Self::TransformedPackData;
        cb = $cb;
        cb(pack_data)
      }
    }
  };
}

macro_rules! generic_config_doc {
  () => {
    "Additional set of optional parameters used by the corresponding request."
  };
}

macro_rules! min_context_slot_doc {
  () => {
    "Set the minimum slot for the request."
  };
}
