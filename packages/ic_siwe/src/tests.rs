#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::settings::SettingsBuilder;

    #[test]
    fn test_global_state_initialization() {
        // Test that global state is properly initialized
        ensure_globals_initialized();

        // Check that all global state is accessible
        assert!(RNG.get().is_some(), "RNG should be initialized");
        assert!(SETTINGS.get().is_some(), "SETTINGS should be initialized");
        assert!(SIWE_MESSAGES.get().is_some(), "SIWE_MESSAGES should be initialized");
    }

    #[test]
    fn test_nonce_generation_in_test_mode() {
        // Test that nonce generation works in test mode (without IC dependencies)
        let nonce_result = crate::rand::generate_nonce();
        assert!(nonce_result.is_ok(), "Nonce generation should work: {:?}", nonce_result);

        let nonce = nonce_result.unwrap();
        assert!(!nonce.is_empty(), "Nonce should not be empty");
        assert_eq!(nonce.len(), 20, "Nonce should be 20 hex characters (10 bytes)");
    }

    #[test]
    fn test_settings_builder_creation() {
        // Test that settings can be created without IC dependencies
        let settings_result = SettingsBuilder::new("test.com", "http://test.com", "test-salt")
            .scheme("http")
            .statement("Test SIWE")
            .chain_id(1)
            .sign_in_expires_in(300_000_000_000)
            .build();

        assert!(
            settings_result.is_ok(),
            "Settings should be created successfully: {:?}",
            settings_result
        );

        let settings = settings_result.unwrap();
        assert_eq!(settings.domain, "test.com");
        assert_eq!(settings.uri, "http://test.com");
        assert_eq!(settings.salt, "test-salt");
        assert_eq!(settings.chain_id, 1);
    }

    #[test]
    fn test_eth_address_creation() {
        // Test Ethereum address creation
        let address_result =
            crate::eth::EthAddress::new("0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed");
        assert!(address_result.is_ok(), "Valid Ethereum address should be created");

        let address = address_result.unwrap();
        assert_eq!(address.as_str(), "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed");
    }

    #[test]
    fn test_eth_address_validation() {
        // Test invalid Ethereum address
        let address_result = crate::eth::EthAddress::new("invalid-address");
        assert!(address_result.is_err(), "Invalid Ethereum address should be rejected");
    }

    // Note: IC-specific initialization tests can only be run in a canister environment
    // These would need to be integration tests or e2e tests
    //
    // For example:
    // - test_initialization_flow() - requires ic_cdk_timers and raw_rand
    // - test_full_siwe_flow() - requires actual IC environment
    // - test_delegation_creation() - requires certified data operations
}
