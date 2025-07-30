// 	Blok yapısı (Block), hash hesaplama ve gösterim.
use crate::tx::Transaction;
use chrono::Utc;
use serde::{Deserialize, Serialize, ser};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(
        index: u64,
        previous_hash: String,
        transactions: Vec<Transaction>,
        difficulty: usize,
    ) -> Self {
        let timestamp = Utc::now().timestamp_millis() as u128;
        let mut nonce = 0;

        loop {
            let serialized_transactions = serde_json::to_string(&transactions).unwrap();
            let hash_input = format!(
                "{}{}{}{}{}",
                index, timestamp, &previous_hash, serialized_transactions, nonce
            );
            let hash_result = calculate_hash(&hash_input);

            // Difficulty: Hash'in başında belirli sayıda '0' olmalı
            if hash_result.starts_with(&"0".repeat(difficulty)) {
                return Block {
                    index,
                    timestamp,
                    previous_hash,
                    hash: hash_result,
                    nonce,
                    transactions,
                };
            }

            nonce += 1;
        }
    }
}

/// Yardımcı fonksiyon: SHA256 hash hesaplar
pub fn calculate_hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}
