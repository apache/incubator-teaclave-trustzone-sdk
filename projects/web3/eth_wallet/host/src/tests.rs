pub mod tests {
    use crate::*;

    pub fn test_workflow() {
        // Simulate the workflow of creating a wallet, deriving an address, and signing a transaction
        let wallet_id = create_wallet().unwrap();
        let address = derive_address(wallet_id, "m/44'/60'/0'/0/0").unwrap();
        let result = sign_transaction(
            wallet_id,
            "m/44'/60'/0'/0/0",
            5,
            0,
            address,
            100,
            1000000000,
            21000,
        );
        assert!(result.is_ok());
    }
}
