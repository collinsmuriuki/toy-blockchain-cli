use serde_derive::Serialize;
use sha2::{Digest, Sha256};
use std::fmt::Write;

#[derive(Debug, Clone, Serialize)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f64,
}

#[derive(Debug, Serialize)]
pub struct BlockHeader {
    timestamp: i64,
    nonce: u32,
    pre_hash: String,
    merkle: String,
    difficulty: u32,
}

pub struct Block {
    header: BlockHeader,
    count: u32,
    transactions: Vec<Transaction>,
}

pub struct Chain {
    chain: Vec<Block>,
    current_transactions: Vec<Transaction>,
    difficulty: u32,
    miner_address: String,
    reward: f64,
}

impl Chain {
    pub fn new(miner_address: String, difficulty: u32) -> Self {
        let mut chain = Chain {
            chain: Vec::new(),
            current_transactions: Vec::new(),
            difficulty,
            miner_address,
            reward: 100.0,
        };
        chain.generate_new_block();
        chain
    }

    pub fn new_transaction(&mut self, sender: String, receiver: String, amount: f64) -> bool {
        self.current_transactions.push(Transaction {
            sender,
            receiver,
            amount,
        });

        true
    }

    fn last_chain(&self) -> String {
        let block = match self.chain.last() {
            Some(block) => block,
            None => return String::from_utf8(vec![48; 64]).unwrap(),
        };
        Chain::hash(&block.header)
    }

    pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
        self.difficulty = difficulty;
        true
    }

    fn update_reward(&mut self, reward: f64) -> bool {
        self.reward = reward;
        true
    }

    fn generate_new_block(&self) {
        unimplemented!()
    }

    fn hash(block_header: &BlockHeader) -> String {
        unimplemented!()
    }
}
