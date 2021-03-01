use sha2::{Sha256, Digest};
use serde::{Serialize};
use bincode;
use rand::Rng;

#[derive(Serialize, Clone, Debug)]
struct TransactionData {
    from: String,
    to: String,
    amount: u32,
}

#[derive(Serialize, Clone, Debug)]
struct Transaction {
    data: TransactionData,
}

#[derive(Serialize, Debug)]
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
    pending_transactions: Vec<Transaction>,
    blocks: Vec<Block>, 
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            pending_transactions: vec![],
            blocks: vec![create_genesis_block()]
        }
    }
        
    fn add_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction); 
    }
    
    fn get_difficulty(&self) -> usize {
        2 as usize
    }
    
    fn mine_pending_transactions(&mut self) {
        let mut block = Block {
            prev_hash: None,
            hash: None,
            nonce: None,
            transactions: self.pending_transactions.clone(),
        };
        
        self.pending_transactions = vec![];
        
        block.mine(self.get_difficulty());
        
        self.blocks.push(block);
    }
    
    fn is_valid(&self) -> bool {
        self.blocks.iter().enumerate().all(|(i, block)| {
            // The genesis block. Always valid.
            if i == 0 {
                return true;
            }
            
            if hashes_equal(&block.hash.as_ref().unwrap().clone(), &block.calculate_hash()) {
                return true;
            }
            
            let prev_block = self.blocks.get(i - 1).unwrap();
            
            // println!("prev_block: #{:?}", prev_block);
            
            // println!("hashes_equal: #{:?}", hashes_equal(&vec![1,2,3], &vec![1,2,3]));
        
            // if vectors_equal(&block.hash.unwrap().as_ref(), &block.calculate_hash()) {
            //     return true;
            // }
            
            false
        })
    }
}

fn hashes_equal(a: &Vec<u8>, b: &Vec<u8>) -> bool {
    println!("hashes_equal: #{:?} #{:?}", a, b);
    
    if a.len() != b.len() {
        return false;
    }
    
    a.iter().zip(b).all(|(a, b)| a == b)
}


fn create_genesis_block() -> Block {
    Block {
        prev_hash: None,
        hash: None,
        nonce: None,
        transactions: vec![],
    }
}

fn generate_nonce() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

fn main() {
    let mut blockchain = Blockchain::new();
    
    let transaction1 = Transaction {
        data: TransactionData {
            from: String::from("rens"),
            to: String::from("betty"),
            amount: 10,
        },
    };
    
    blockchain.add_transaction(transaction1);
    blockchain.mine_pending_transactions();
    
    println!("Blockchain valid? #{:?}", blockchain.is_valid());
}
