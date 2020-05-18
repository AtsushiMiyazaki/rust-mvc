use super::*;
use ed25519_dalek::PublicKey;

#[derive(Debug)]
pub struct Blockchain {
    blocks: Vec<Block>,
    transaction_pool: Vec<Transaction>,
    mining_reward: f32,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            blocks: vec![],
            transaction_pool: vec![],
            mining_reward: MINING_REWARD
        }
    }

    pub fn do_mining(&mut self, miner_address: PublicKey) {
        self.transaction_pool.push(Transaction{
            sender: None,
            receiver: Some(miner_address),
            amount: self.mining_reward,
            signature: None
        });
        let unmined_transactions = self.transaction_pool.clone();
        let mut block = Block::new(unmined_transactions);
        match self.blocks.last() {
            Some(pre_block) => block.set_pre_hash(pre_block.hash.to_owned()),
            None => block.set_pre_hash("genesis_block".to_owned())
        }
        block.set_hash();
        block.mine();
        self.blocks.push(block);
        self.transaction_pool = vec![];
    }

    pub fn add_new_transaction(&mut self, transaction: Transaction) {
        if transaction.sender.is_none() || transaction.receiver.is_none() {
            panic!("Sender and receiver must be specified")
        }

        if transaction.is_valid_transaction() == false {
            panic!("Invalid transaction")
        }

        if self.is_balance_enough(&transaction) == false {
            panic!("Sender balance not enough")
        }

        self.transaction_pool.push(transaction);
    }

    fn is_balance_enough(&self, new_transaction: &Transaction) -> bool {
        let mut user_balance: f32 = 0f32;
        let blocks = &self.blocks;
        let sender = &new_transaction.sender;

        for (_, block) in blocks.iter().enumerate() {
            for (_, transaction) in block.transaction.iter().enumerate() {
                match transaction.receiver {
                    sender => user_balance += transaction.amount,
                    _ => {}
                }
            }
        }
        
        println!("Sender balance: {}, sending amount: {}", user_balance, new_transaction.amount);
        if user_balance < new_transaction.amount {
            return false
        }

        return true
    }

    pub fn is_valid_chain(&self) -> bool {
        let blocks = &self.blocks;

        for (i, block) in blocks.iter().enumerate() {
            if block.hash != calculate_hash(
                &block.pre_hash,
                &block.transaction,
                &block.timestamp,
                &block.nonce,
            )
            {
                return false
            }
            if i > 0 && block.pre_hash != blocks[i-1].hash {
                return  false
            }
        }

        return true
    }
}
