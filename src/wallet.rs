use ed25519_dalek::{Keypair, PublicKey, SecretKey};
use rand::rngs::OsRng;

#[derive(Debug)]
pub struct Wallet {
    pub pub_key: PublicKey,
    pub sec_key: SecretKey,
}

impl Wallet {
    pub fn new() -> Self {
        let mut csprng = OsRng{};
        let keypair = Keypair::generate(&mut csprng);
        Wallet {
            pub_key: keypair.public,
            sec_key: keypair.secret,
        }
    }
}
