use super::Token;
use alloy::primitives::{Address, U256};
use anyhow::Result;
use log::{debug, error, info};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct TokenRegistry {
    tokens: HashMap<Address, Token>,
    network_id: u64,
}

impl TokenRegistry {
    pub fn new(network_id: u64) -> Self {
        Self {
            tokens: HashMap::new(),
            network_id,
        }
    }

    /// Create a new TokenRegistry with network ID
    pub fn with_network_id(network_id: u64) -> Self {
        Self {
            tokens: HashMap::new(),
            network_id,
        }
    }

    /// Set network ID for this registry
    pub fn set_network_id(&mut self, network_id: u64) {
        self.network_id = network_id;
    }

    /// Get network ID
    pub fn get_network_id(&self) -> u64 {
        self.network_id
    }

    pub fn to_raw_amount(&self, address: Address, amount: &str) -> Result<U256> {
        self.tokens.get(&address).unwrap().to_raw_amount(amount)
    }

    pub fn to_human_amount(&self, address: Address, amount: U256) -> Result<String> {
        self.tokens.get(&address).unwrap().to_human_amount(amount)
    }

    pub fn to_raw_amount_f64(&self, address: Address, amount: f64) -> Result<U256> {
        self.tokens.get(&address).unwrap().to_raw_amount_f64(amount)
    }

    pub fn to_human_amount_f64(&self, address: Address, amount: U256) -> Result<f64> {
        self.tokens
            .get(&address)
            .unwrap()
            .to_human_amount_f64(amount)
    }

    pub fn add_token(&mut self, token: Token) {
        info!("Token {}", token);
        self.tokens.insert(token.address, token);
    }

    /// Add a token with automatic network_id assignment from registry
    pub fn add_token_with_network_id(&mut self, mut token: Token) -> Result<()> {
        token.network_id = self.network_id;
        self.tokens.insert(token.address, token);
        Ok(())
    }

    pub fn get_token(&self, address: Address) -> Option<&Token> {
        self.tokens.get(&address)
    }

    pub fn get_token_mut(&mut self, address: Address) -> Option<&mut Token> {
        self.tokens.get_mut(&address)
    }

    pub fn remove_token(&mut self, address: Address) -> Option<Token> {
        self.tokens.remove(&address)
    }

    pub fn contains_token(&self, address: Address) -> bool {
        self.tokens.contains_key(&address)
    }

    pub fn get_all_tokens(&self) -> Vec<&Token> {
        self.tokens.values().collect()
    }

    /// Get total token count
    pub fn token_count(&self) -> usize {
        self.tokens.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::address;

    #[test]
    fn test_token_registry() {
        let mut registry = TokenRegistry::new(1);

        // Test network ID functionality
        registry.set_network_id(1);
        assert_eq!(registry.get_network_id(), 1);

        let token = Token::new(
            address!("0x1f9840a85d5af5bf1d1762f925bdaddc4201f984"), // UNI token
            1,                                                      // Ethereum mainnet
            "UNI".to_string(),
            "Uniswap".to_string(),
            18,
        );

        // Test adding token
        registry.add_token(token.clone());
        assert!(registry.contains_token(token.address));

        // Test retrieving token
        let retrieved_token = registry.get_token(token.address).unwrap();
        assert_eq!(retrieved_token.symbol, "UNI");
        assert_eq!(retrieved_token.decimals, 18);
        assert_eq!(retrieved_token.name, "Uniswap");

        // Test removing token
        let removed_token = registry.remove_token(token.address).unwrap();
        assert_eq!(removed_token.symbol, "UNI");
        assert!(!registry.contains_token(token.address));

        assert_eq!(registry.token_count(), 0);
    }
}
