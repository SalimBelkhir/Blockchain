use sha2::{Sha256, Digest};
use solana_sdk::clock::UnixTimestamp;
use std::time::{SystemTime,UNIX_EPOCH};

pub fn hash_data(data:&[u8])->Vec<u8>{
    let mut hasher = Sha256::new();
    hasher.update(&data);
    let msg_hash = hasher.finalize().to_vec() ;
    return msg_hash ;
}


//Creating a Blockchain
#[derive(Debug)]
pub struct Block{
    index : u64,
    timestamp: u64,
    data: String,
    previous_hash : Vec<u8>,
    hash: Vec<u8>,
}

impl Block {
    pub fn new(index:u64,data:String,previous_hash:Vec<u8>)-> Self{
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let mut block =Block { index, 
            timestamp, 
            data, 
            previous_hash,
            hash : Vec::new(),
        } ;
        block.hash = block.calculate_hash();
        block 
    }

    pub fn calculate_hash(&self)->Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(self.index.to_be_bytes());
        hasher.update(self.timestamp.to_be_bytes());
        hasher.update(&self.data);
        hasher.update(&self.previous_hash);
        hasher.finalize().to_vec() 
    }
}
#[derive(Debug)] // to make you display structs
pub struct Blockchain{
    chain : Vec<Block>,
}

 impl Blockchain{
    pub fn new() -> Self {
        let genesis_block = Block::new(0,"Genesis Block".to_string(),Vec::new()) ;
        Blockchain{
            chain:vec![genesis_block],
        }
    }
    pub fn add_block(&mut self,data:String){
            let previous_block = self.chain.last().unwrap() ;
            let new_block = Block::new(previous_block.index+1, data, previous_block.hash.clone());
            self.chain.push(new_block);
        }
    }
