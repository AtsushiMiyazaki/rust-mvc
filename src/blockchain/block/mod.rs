extern crate crypto;

use chrono::Utc;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use hex;

pub struct BlockHeader {
    previous_hash: String,
    merkle_root: String,
    timestamp: i64,
    nonce: i64,
}

pub struct Block {
    pub block_header: BlockHeader,
    pub height: i64,
    pub hash: String,
    pub transactions: Vec<String>,
}

pub trait BlockT {
    fn new(height: i64, merkle_root: String, previous_hash: String) -> Block;
    fn calculate_block_header_hash(&mut self);
    fn to_json(&self) -> Block;
}

impl BlockT for Block {
    fn new(height: i64, previous_hash: String, merkle_root: String) -> Block {
        Block {
            block_header: BlockHeader {
                previous_hash,
                merkle_root,
                timestamp: Utc::now().timestamp_millis(),
                nonce: 0,
            },
            height,
            hash: String::new(),
            transactions: Vec::new(),
        }
    }

    /// This function calculates block header hash
    fn calculate_block_header_hash(&mut self) {
        let mut sha256 = Sha256::new();

        let block_header_hex_str = format!(
            "{}{}{:x}{:x}",
            &self.block_header.previous_hash,
            &self.block_header.merkle_root,
            &self.block_header.timestamp,
            &self.block_header.nonce
        );

        let block_header_bin: Vec<u8>;
        match hex::decode(block_header_hex_str) {
            Ok(value) => block_header_bin = value,
            Err(_) => {
                println!("ERROR");
                return;
            }
        };

        sha256.input(&block_header_bin);
        let block_header_hash = sha256.result_str();
        self.hash = block_header_hash;
    }
    
    fn to_json(&self) -> Block {
        Block {
            block_header: BlockHeader {
                previous_hash: self.block_header.previous_hash.clone(),
                merkle_root: self.block_header.merkle_root.clone(),
                timestamp: self.block_header.timestamp.clone(),
                nonce: self.block_header.nonce.clone(),
            },
            height: self.height.clone(),
            hash: self.hash.clone(),
            transactions: Vec::new(),
        }
    }
}
