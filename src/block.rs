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