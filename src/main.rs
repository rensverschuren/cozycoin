use sha2::{Sha256, Digest};
use serde::{Serialize};
use bincode;
use rand::Rng;

#[derive(Serialize)]
struct TransactionData {
    from: String,
    to: String,
    amount: u32,
}

#[derive(Serialize)]
struct Transaction {
    data: TransactionData,
}

#[derive(Serialize)]
struct Block {
    hash: Option<Vec<u8>>,
    prev_hash: Option<Vec<u8>>,
    nonce: Option<u32>,
    transactions: Vec<Transaction>,
}

impl Block {
    fn calculate_hash(&self) -> Vec<u8> {
        let serialized = &bincode::serialize(&self).unwrap();
        
        Sha256::digest(&serialized).to_vec()
    }
    
    fn calculate_nonce_hash(&mut self) {
        self.nonce = Some(generate_nonce());
        self.hash = Some(self.calculate_hash());
    }
    
    fn mine(&mut self, difficulty: usize) -> () {
        println!("Mining block...");
        
        self.calculate_nonce_hash();
        
        while !&self.hash.as_ref().unwrap()[0..difficulty].iter().all(|&x| x == 0) {
            self.calculate_nonce_hash();
        }
        
        println!("Block mined with hash: #{:?}", self.hash);
    }
}

struct Blockchain {
    blocks: Vec<Block>, 
}

fn generate_nonce() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

fn main() {
    let blockchain = Blockchain {
        blocks: vec![],
    };
    
    let transaction_data_1 = TransactionData {
        from: String::from("rens"),
        to: String::from("betty"),
        amount: 10,
    };
    
    let transaction1 = Transaction {
        data: transaction_data_1,
    };
    
    let mut block1 = Block {
        prev_hash: None,
        hash: None,
        nonce: None,
        transactions: vec![]
    };
    
    block1.transactions.push(transaction1);
    
    let hash = block1.mine(2);
}
