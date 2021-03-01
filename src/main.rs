use bincode;
use rand::Rng;
use serde::Serialize;
use sha2::{Digest, Sha256};

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
    #[serde(skip_serializing)]
    hash: Option<[u8; 32]>,
    prev_hash: Option<[u8; 32]>,
    nonce: Option<u32>,
    transactions: Vec<Transaction>,
}

impl Block {
    fn calculate_hash(&self) -> [u8; 32] {
        let serialized = &bincode::serialize(&self).unwrap();

        let hash = Sha256::digest(&serialized);
        let hash_value = hash.as_slice();

        [
            hash_value[0],
            hash_value[1],
            hash_value[2],
            hash_value[3],
            hash_value[4],
            hash_value[5],
            hash_value[6],
            hash_value[7],
            hash_value[8],
            hash_value[9],
            hash_value[10],
            hash_value[11],
            hash_value[12],
            hash_value[13],
            hash_value[14],
            hash_value[15],
            hash_value[16],
            hash_value[17],
            hash_value[18],
            hash_value[19],
            hash_value[20],
            hash_value[21],
            hash_value[22],
            hash_value[23],
            hash_value[24],
            hash_value[25],
            hash_value[26],
            hash_value[27],
            hash_value[28],
            hash_value[29],
            hash_value[30],
            hash_value[31],
        ]
    }

    fn calculate_nonce_hash(&mut self) {
        self.nonce = Some(generate_nonce());
        self.hash = Some(self.calculate_hash());
    }

    fn mine(&mut self, difficulty: usize) -> () {
        println!("Mining block...");

        self.calculate_nonce_hash();

        while !&self.hash.as_ref().unwrap()[0..difficulty]
            .iter()
            .all(|&x| x == 0)
        {
            self.calculate_nonce_hash();
        }

        println!("Block mined: #{:?}", self);
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
            blocks: vec![create_genesis_block()],
        }
    }

    fn add_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }

    fn get_difficulty(&self) -> usize {
        2 as usize
    }

    fn get_last_block(&self) -> &Block {
        self.blocks.last().unwrap()
    }

    fn mine_pending_transactions(&mut self) {
        let mut block = Block {
            prev_hash: self.get_last_block().hash,
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

            if block.hash.is_none() || block.prev_hash.is_none() {
                return false;
            }

            if !hashes_equal(block.hash.unwrap(), block.calculate_hash()) {
                return false;
            }

            let prev_block = self.blocks.get(i - 1).unwrap();

            if !hashes_equal(prev_block.hash.unwrap(), block.prev_hash.unwrap()) {
                return false;
            }

            true
        })
    }
}

fn hashes_equal(a: [u8; 32], b: [u8; 32]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    a.iter().zip(&b).all(|(a, b)| a == b)
}

fn create_genesis_block() -> Block {
    Block {
        prev_hash: None,
        hash: Some([
            0, 0, 109, 38, 208, 184, 118, 135, 170, 255, 119, 83, 34, 112, 8, 69, 36, 88, 154, 20,
            28, 55, 170, 18, 250, 85, 6, 238, 98, 84, 24, 154,
        ]),
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
