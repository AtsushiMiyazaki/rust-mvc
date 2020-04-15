mod block;

use block::BlockT;

pub fn init() {
    let mut b = block::Block::new(
        0,
        String::from("000ea49e16b863b5e74591562719c7d76f8992b69ddaf00a8a01b490c5adae41"),
        String::from("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
    );

    b.calculate_block_header_hash();
    println!("block hash is {}", b.hash);
    let block_json = b.to_json();
    println!("block hash is {}", block_json.hash);
    println!("block height is {}", block_json.height);
}
