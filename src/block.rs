extern crate bincode;
use bincode::{serialize, deserialize};
use crate::{ProofOfWork, Transaction};
use serde::{Deserialize, Serialize};

pub struct Block {
    timestamp: i64,
    pre_block_hash: string,
    hash: string,
    transactions: vec<Transactions>,
    nonce: i64,
    height: usize,
}

pub fn new_block(pre_block_hash: String, transactions: &[Transactions], height: usize) -> Block{
    let mut Block = Block {
        timestamp: crate::current_timestamp(),
        pre_block_hash,
        hash: String::new(),
        transactions: transactions.to_vec(),
        nonce: 0,
        height,


    };

    let pow = proofofwork::new_proof_of_work(block.clone());
    let (nonce, hash) = pow.run();
    block.nonce = nonce;
    block.hash = hash;
    return block

}

impl Block{
    // Functions related to the Block type can be implemented here
    pub fn new_block(pre_block_hash: String, transactions: &[Transaction],
        height: usize) -> Block {
            let mut block = Block{
                timestamp: crate::current_timestamp(),
                pre_block_hash,
                hash: string::new(),
                transactions: transactions.to_vec(),
                nonce: 0,
                height,

            };
            let pow = ProofOfWork::new_proof_of_work(block.clone());
            let (nonce, hash) = pow.run();
            block.nonce = nonce;
            block.hash = hash;
            return block;

        }

        pub fn deserialize(bytes: &[u8]) -> Block {
            bincode::deserialize(bytes).unwrap()
        }
        pub fn serialize(&self) -> Vec<u8> {
            bincode::serialize(&self).unwrap().to_vec()
        }
    
    }

    