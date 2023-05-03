use sha3::{Digest, Sha3_512};

pub fn calc_hash(data: &str) -> String {

    let mut hasher = Sha3_512::new();
    hasher.update(data);
    format!("{:X}", hasher.finalize())
}