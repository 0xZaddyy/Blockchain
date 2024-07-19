pub struct Transaction {
    id : Vec <u8>,
    vin: Vec<Txinput>,
    vout: Vec<Txouput>,
}

pub struct Txinput {
    txid: Vec<u8>,
    vout: usize,
    signature: Vec<u8>,
    pubkey: Vec<u8>,

}

pub struct Txoutput {
    value: i32,
    pub_key_hash: Vec<u8>,
}