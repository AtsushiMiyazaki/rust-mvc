use rustmvblib::*;
use ed25519_dalek::Keypair;

fn main() {
    let mut blockchain = Blockchain::new();

    let bob = Wallet::new();
    let miner_a = Wallet::new();

    blockchain.do_mining(miner_a.pub_key);

    let mut first_transaction = Transaction {
        sender: Some(miner_a.pub_key),
        receiver: Some(bob.pub_key),
        amount: 100.0,
        signature: None,
    };

    first_transaction.sign_transaction(Keypair {
        public: miner_a.pub_key,
        secret: miner_a.sec_key,
    });

    blockchain.add_new_transaction(first_transaction);
    blockchain.do_mining(miner_a.pub_key);

    println!("{}", blockchain.is_valid_chain());
    println!("Blockchain: {:#?}", blockchain);
}
