use sha2::digest::{crypto_common::BlockSizeUser, FixedOutputReset, HashMarker, Digest};
use starknet_types_core::felt::Felt;
use crate::misc::U256;
use wtx::misc::memset_slice_volatile;

const EC_ORDER: U256 =
    U256::from_be_hex("0800000000000010ffffffffffffffffb781126dcae7b2321e66a241adc64d2f");


pub fn generate_k(message_hash: &Felt, private_key: &Felt, seed: Option<&Felt>) -> Felt {
    let message_hash = U256::from_be_slice(&message_hash.to_bytes_be()).to_be_byte_array();
    let private_key = U256::from_be_slice(&private_key.to_bytes_be());

    let seed_bytes = match seed {
        Some(seed) => seed.to_bytes_be(),
        None => [0u8; 32],
    };

    let mut first_non_zero_index = 32;
    for (ind, element) in seed_bytes.iter().enumerate() {
        if *element != 0u8 {
            first_non_zero_index = ind;
            break;
        }
    }

    let mut k = generate_k_shifted::<sha2::Sha256, _>(
        &private_key,
        &EC_ORDER,
        &message_hash,
        &seed_bytes[first_non_zero_index..],
    );
    let rslt = Felt::from_bytes_be(&k);
    memset_slice_volatile(&mut k, 0);
    rslt
}

#[inline]
fn generate_k_shifted<D, I>(x: &mut [u8], n: &[u8], h: &[u8], data: &[u8]) -> [u8; 32]
where
    D: Default + Digest + BlockSizeUser + FixedOutputReset + HashMarker,
{
    let mut hmac_drbg = rfc6979::HmacDrbg::<D>::new(&x, h, data);
    memset_slice_volatile(x, 0);
    loop {
        let mut bytes = [0; 32];
        hmac_drbg.fill_bytes(&mut bytes);
        let k = U256::from_be_byte_array(bytes) >> 4;
        if (!k.is_zero() & k.ct_lt(n)).into() {
            return k.to_be_bytes();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::field_element_from_be_hex;
    #[cfg(not(feature = "std"))]
    use alloc::vec::Vec;

    use serde::Deserialize;

    #[derive(Deserialize)]
    struct Rfc6979TestVecotr<'a> {
        msg_hash: &'a str,
        priv_key: &'a str,
        seed: &'a str,
        k: &'a str,
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_generate_k_padded() {
        // Test vectors generated from `cairo-lang`
        test_generate_k_from_json_str(include_str!("../test-data/rfc6979_padded.json"));
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_generate_k_not_padded() {
        // Test vectors generated from `cairo-lang`
        test_generate_k_from_json_str(include_str!("../test-data/rfc6979_not_padded.json"));
    }

    fn test_generate_k_from_json_str(json_str: &'static str) {
        let test_vectors: Vec<Rfc6979TestVecotr<'_>> = serde_json::from_str(json_str).unwrap();

        for test_vector in &test_vectors {
            let msg_hash = field_element_from_be_hex(test_vector.msg_hash);
            let priv_key = field_element_from_be_hex(test_vector.priv_key);
            let seed = field_element_from_be_hex(test_vector.seed);
            let expected_k = field_element_from_be_hex(test_vector.k);

            let k = generate_k(&msg_hash, &priv_key, Some(&seed));

            assert_eq!(k, expected_k);
        }
    }
}
