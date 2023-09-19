use crate::util::hash;
use crate::transaction::Transaction;

#[derive(Debug)]
pub struct Block {
    pub transactions: Vec<Transaction>,
    pub previous_hash: Option<String>,
    pub hash: String,
}

pub struct Blockchain {
    pub chain: Vec<Block>
}

impl Block {
    fn generate_content(&self) -> String {
        let concatenated_txs = self.transactions.iter()
            .map(|tx| format!("{:?}", tx))
            .collect::<Vec<_>>()
            .join(",");
        format!("{}{}", concatenated_txs, self.previous_hash.clone().unwrap_or_default())
    }
    pub fn new(transactions: Vec<Transaction>, previous_hash: Option<String>) ->Self {
        let content = Self { transactions: transactions.clone(), previous_hash: previous_hash.clone(), hash: String::new() }.generate_content();
        let hash = hash(&content);
        Self { transactions, previous_hash, hash }
    }

    pub fn is_valid(&self, previous_block: &Option<&Block>) -> bool {
        if let Some(prev_block) = previous_block {
            if prev_block.hash != *self.previous_hash.as_ref().unwrap() {
                return false;
            }
        }
        let content = self.generate_content();
        hash(&content) == self.hash
    }
}

impl Blockchain {
    pub fn new() -> Self {
        Self { chain: vec![]}
    }

    pub fn add_block(&mut self, block: Block) {
        self.chain.push(block)
    }
}
