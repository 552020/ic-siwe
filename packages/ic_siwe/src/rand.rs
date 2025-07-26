#[cfg(not(test))]
pub(crate) fn generate_nonce() -> Result<String, String> {
    use crate::RNG;
    use rand_chacha::rand_core::RngCore;

    let mut buf = [0u8; 10];
    RNG.with_borrow_mut(|rng| match rng.as_mut() {
        Some(r) => {
            r.fill_bytes(&mut buf);
            Ok(hex::encode(buf))
        }
        None => Err("RNG not initialized".to_string()),
    })
}

#[cfg(test)]
pub(crate) fn generate_nonce() -> Result<String, String> {
    use rand::{thread_rng, Rng};

    let mut rng = thread_rng();
    let mut nonce = [0u8; 10];
    rng.fill(&mut nonce);
    Ok(hex::encode(nonce))
}
