// Zinciri (Blockchain) ve zincire işlem/blok ekleme mantığı.
use crate::block::{Block, calculate_hash};
use crate::tx::Transaction;
use std::fs;
use std::path::Path;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            difficulty,
        };

        let genesis_block = Block::new(
            0,
            String::from("0"),
            Vec::<Transaction>::new(), // ya da  vec![] veya Vec::new()
            difficulty,
        );

        blockchain.chain.push(genesis_block);

        blockchain
    }

    // Zincire yeni blok ekler
    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let latest_block = self.chain.last().expect("Zincirde hiç blok yok");
        let new_block = Block::new(
            latest_block.index + 1,
            latest_block.hash.clone(),
            transactions,
            self.difficulty,
        );

        self.chain.push(new_block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];
            let serialized_transactions = serde_json::to_string(&current.transactions).unwrap();

            let expected_hash = calculate_hash(&format!(
                "{}{}{}{}{}",
                current.index,
                current.timestamp,
                current.previous_hash,
                serialized_transactions,
                current.nonce
            ));

            if current.hash != expected_hash {
                return false;
            }

            if current.previous_hash != previous.hash {
                return false;
            }
        }

        true
    }

    pub fn print_chain(&self) {
        for block in &self.chain {
            println!("{:#?}", block);
        }
    }

    pub fn save_to_file(&self, filename: &str) {
        let json = serde_json::to_string_pretty(&self.chain).unwrap();
        fs::write(filename, json).expect("Dosya yazma başarısız oldu!!");
    }

    pub fn load_from_file(filename: &str, difficulty: usize) -> Self {
        if Path::new(filename).exists() {
            let content = fs::read_to_string(filename).expect("Dosya okuma başarısız");
            let chain: Vec<Block> = serde_json::from_str(&content).expect("Json çözülemedi");
            Blockchain { chain, difficulty }
        } else {
            Blockchain::new(difficulty)
        }
    }
}
