// 	Blok yapısı (Block), hash hesaplama ve gösterim.
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
    pub data: String, // şimdilik basit tutalım
}

impl Block {
    pub fn new(index: u64, previous_hash: String, data: String, difficulty: usize) -> Self {
        let timestamp = Utc::now().timestamp_millis() as u128;
        let mut nonce = 0;

        loop {
            let hash_input = format!("{}{}{}{}{}", index, timestamp, &previous_hash, &data, nonce);
            let hash_result = calculate_hash(&hash_input);

            // Difficulty: Hash'in başında belirli sayıda '0' olmalı
            if hash_result.starts_with(&"0".repeat(difficulty)) {
                return Block {
                    index,
                    timestamp,
                    previous_hash,
                    hash: hash_result,
                    nonce,
                    data,
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
