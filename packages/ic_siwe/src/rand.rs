#[cfg(not(test))]
pub(crate) fn generate_nonce() -> Result<String, String> {
    use crate::{ensure_globals_initialized, RNG};
    use rand_chacha::rand_core::RngCore;

    // Ensure global state is initialized
    ensure_globals_initialized();

    let mut buf = [0u8; 10];
    RNG.get()
        .expect("RNG global state should be initialized")
        .write()
        .unwrap()
        .as_mut()
        .map(|rng| {
            rng.fill_bytes(&mut buf);
            hex::encode(buf)
        })
        .ok_or_else(|| "RNG not initialized".to_string())
}

#[cfg(test)]
pub(crate) fn generate_nonce() -> Result<String, String> {
    use rand::{thread_rng, Rng};

    let mut rng = thread_rng();
    let mut nonce = [0u8; 10];
    rng.fill(&mut nonce);
    Ok(hex::encode(nonce))
}
