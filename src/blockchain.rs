// Zinciri (Blockchain) ve zincire işlem/blok ekleme mantığı.
use crate::block::{Block, calculate_hash};

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
            String::from("Genesis Block"),
            difficulty,
        );

        blockchain.chain.push(genesis_block);

        blockchain
    }

    // Zincire yeni blok ekler
    pub fn add_block(&mut self, data: String) {
        let latest_block = self.chain.last().expect("Zincirde hiç blok yok");
        let new_block = Block::new(
            latest_block.index + 1,
            latest_block.hash.clone(),
            data,
            self.difficulty,
        );

        self.chain.push(new_block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            let expected_hash = calculate_hash(&format!(
                "{}{}{}{}{}",
                current.index,
                current.timestamp,
                current.previous_hash,
                current.data,
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
}
