

pub struct Transaction {
    tx_id: String,
    sender_public_key: String,
    recipient_public_key: String,
    value: f64,
    timestamp: u64,
    tx_inputs: Vec<TxInput> ,
    tx_outputs: Vec<TxOutput> ,
    signature: String
}

pub struct TxInput {
    tx_id: String,
    index: u64,
}

pub struct TxOutput {
    amount: f64,
    index: u64,
    recipient_public_key: String
}

pub trait TransactionT { 
    fn new (
        tx_id: String,
        sender_public_key: String,
        recipient_public_key: String,
        value: f64,
        timestamp: u64,
        tx_inputs: Vec<TxInput> ,
        tx_outputs: Vec<TxOutput> ,
        signature: String
    ) -> Self;

    fn calc_tx_hash (&mut self) -> Self;
    fn create_signature (&self) -> Self;
    fn to_json (&self) -> Self;
    fn from_json (&mut self) -> Self;
}

impl TransactionT for Transaction { 
    fn new (        
        tx_id: String,
        sender_public_key: String,
        recipient_public_key: String,
        value: f64,
        timestamp: u64,
        tx_inputs: Vec<TxInput> ,
        tx_outputs: Vec<TxOutput> ,
        signature: String
    ) -> Self {
        Transaction {
            tx_id,
            sender_public_key,
            recipient_public_key,
            value,
            timestamp,
            tx_inputs,
            tx_outputs,
            signature
        }
    }
}
