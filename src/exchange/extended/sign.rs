pub fn sign(private_key: &Felt, message: &Felt, k: &Felt) -> Result<ExtendedSignature, SignError> {
    if message >= &ELEMENT_UPPER_BOUND {
        return Err(SignError::InvalidMessageHash);
    }
    if k == &Felt::ZERO {
        return Err(SignError::InvalidK);
    }

    let full_r = mul_by_bits(&GENERATOR, k).to_affine().unwrap();
    let r = full_r.x();
    if r == Felt::ZERO || r >= ELEMENT_UPPER_BOUND {
        return Err(SignError::InvalidK);
    }

    let k_inv = mod_inverse(k, &EC_ORDER);

    let s = mul_mod_floor(&r, private_key, &EC_ORDER);
    let s = add_unbounded(&s, message);
    let s = bigint_mul_mod_floor(s, &k_inv, &EC_ORDER);
    if s == Felt::ZERO || s >= ELEMENT_UPPER_BOUND {
        return Err(SignError::InvalidK);
    }

    Ok(ExtendedSignature {
        r,
        s,
        v: (full_r.y().to_bigint() & Felt::ONE.to_bigint()).into(),
    })
}