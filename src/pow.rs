// Hash üretiminde kullanılan Proof of Work algoritması.

use crate::{block::calculate_hash, tx::Transaction};

pub fn mine_block(
    index: u64,
    timestamp: u128,
    previous_hash: &str,
    transactions: &Vec<Transaction>,
    difficulty: usize,
) -> (i64, String) {
    let mut nonce: i64 = 0;
    loop {
        let serialized_transactions = serde_json::to_string(&transactions).unwrap();
        let hash_input = format!(
            "{}{}{}{}{}",
            index, timestamp, &previous_hash, serialized_transactions, nonce
        );
        let hash_result = calculate_hash(&hash_input);

        // Difficulty: Hash'in başında belirli sayıda '0' olmalı
        if hash_result.starts_with(&"0".repeat(difficulty)) {
            return (nonce, hash_result);
        }

        nonce += 1;
    }
}
