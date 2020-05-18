use super::*;
#[derive(Debug)]
pub struct Block {
    pub timestamp:u64,
    pub hash: String,
    pub pre_hash: String,
    pub transaction: Vec<Transaction>,
    pub nonce: u64,
}

impl Block {
    pub fn new(transaction: Vec<Transaction>) -> Self {
        let time = now();
        let nonce = 0u64;
        Block {
            timestamp: time,
            hash: String::new(),
            pre_hash: String::new(),
            transaction,
            nonce
        }
    }

    pub fn set_hash(&mut self) {
        self.hash = calculate_hash(
            &self.pre_hash,
            &self.transaction,
            &self.timestamp,
            &self.nonce,
        )
    }

    pub fn set_pre_hash(&mut self, pre_hash: String) {
        self.pre_hash = pre_hash;
    }

    pub fn mine(&mut self) {
        let difficulty_target = get_difficulty_str();

        while &self.hash[..DIFFICULTY as usize] != difficulty_target {
            self.nonce += 1;
            self.set_hash()
        }

        println!("Mining succeeded");
    }

    pub fn has_valid_transactions(&self) -> bool {
        for tx in &self.transaction {
            if tx.is_valid_transaction() == false {
                return false
            }
        }

        return true
    }
}
