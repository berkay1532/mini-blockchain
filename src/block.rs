// 	Blok yapısı (Block), hash hesaplama ve gösterim.
use crate::pow::mine_block;
use crate::tx::Transaction;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: i64,
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

        loop {
            let (nonce, hash_result) =
                mine_block(index, timestamp, &previous_hash, &transactions, difficulty);

            return Block {
                index,
                timestamp,
                previous_hash,
                hash: hash_result,
                nonce,
                transactions,
            };
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
