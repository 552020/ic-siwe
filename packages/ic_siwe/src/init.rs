use crate::settings::Settings;

/// Initializes the SIWE library with the provided settings. Must be called before any other SIWE functions. Use the [SettingsBuilder](crate::settings::SettingsBuilder)  to create a [Settings] object.
///
/// # Parameters
///
/// * `settings` - The SIWE settings to be initialized.
///
/// # Examples
///
/// ```ignore
/// use ic_siwe::{init, settings::SettingsBuilder};
///
/// let settings = SettingsBuilder::new("example.com", "http://example.com", "salt")
///   .scheme("https")
///   .statement("Sign in with Ethereum")
///   .chain_id(1)
///   .sign_in_expires_in(300_000_000_000) // 5 minutes in nanoseconds
///   .build()
///   .unwrap();
///
/// init(settings).unwrap();
/// ```
///
pub fn init(settings: Settings) -> Result<(), String> {
    use crate::{ensure_globals_initialized, SETTINGS};

    // Ensure global state is initialized
    ensure_globals_initialized();

    // Set the settings in the global state
    SETTINGS
        .get()
        .expect("SETTINGS global state should be initialized")
        .write()
        .unwrap()
        .replace(settings);

    init_rng();

    Ok(())
}

fn init_rng() {
    use crate::{ensure_globals_initialized, RNG};
    use candid::Principal;
    use rand_chacha::{rand_core::SeedableRng, ChaCha20Rng};
    use std::time::Duration;

    // Ensure global state is initialized
    ensure_globals_initialized();

    ic_cdk::println!("🚀 Setting up RNG timer callback...");

    // Use timer-based initialization to avoid init mode restrictions
    ic_cdk_timers::set_timer(Duration::ZERO, || {
        ic_cdk::println!("⏰ Timer callback triggered!");
        ic_cdk::spawn(async {
            ic_cdk::println!("🔧 Starting RNG initialization...");
            match ic_cdk::call(Principal::management_canister(), "raw_rand", ()).await {
                Ok((seed,)) => {
                    ic_cdk::println!("✅ Got raw_rand response, initializing RNG...");
                    RNG.get()
                        .expect("RNG global state should be initialized")
                        .write()
                        .unwrap()
                        .replace(ChaCha20Rng::from_seed(seed));
                    ic_cdk::println!("🎉 RNG initialization complete!");
                }
                Err(e) => {
                    ic_cdk::println!("❌ Failed to get raw_rand: {:?}", e);
                }
            }
        })
    });

    ic_cdk::println!("📅 Timer callback scheduled successfully");
}
