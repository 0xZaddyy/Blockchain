pub struct proofofwork {
    block: Block,
    target:Bigint,
}

pub fn run(&self) -> (i64, String) {
    let mut nonce = 0;
    let mut hash = Vec::new();
    println!("mining the block");
    while nonce < MAX_NONCE {
        let data = self.prepare_data(nonce);
        hash = crate::SHA256_digest(data.as_slice());
        let hash_int = Bigint::from_bytes_be(sign::plus, hash.as_slice());

        if hash_int.lt(self.target.borrow()) {
            println!("{}" HEXLOWER.encode(hash.as_slice()));
            break;
    
        } else {
            nonce += 1 ;
        }
    }
    println!();
    return(nonce,HEXLOWER.encode(hash.as_slice()));


}

    
    
