mod block;
pub use block::Block;
pub mod blockchain;
pub use blockchain::Blockchain;
pub mod transaction;
pub use transaction::Transaction;
pub mod wallet;
pub use wallet::Wallet;
use std::time::{SystemTime, UNIX_EPOCH};

const DIFFICULTY: i32 = 3;
const MINING_REWARD: f32 = 10000f32;

pub fn now() -> u64 {
    let start = SystemTime::now();
    let time_since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Failed to get the time");
    time_since_the_epoch.as_secs()
}

pub fn calculate_hash(
    pre_hash: &String,
    transactions: &Vec<Transaction>,
    timestamp: &u64,
    nonce: &u64
) -> String {
    let mut bytes = vec![];
    bytes.extend(&timestamp.to_ne_bytes());
    bytes.extend(transactions.iter().flat_map(|transaction|transaction.to_bytes()).collect::<Vec<u8>>());
    bytes.extend(pre_hash.as_bytes());
    bytes.extend(&nonce.to_ne_bytes());

    crypto_hash::hex_digest(crypto_hash::Algorithm::SHA256, &bytes)
}

pub fn get_difficulty_str() -> String {
    let mut s = String::new();
    for _i in 0..DIFFICULTY {
        s.push_str("0");
    }

    s
}
