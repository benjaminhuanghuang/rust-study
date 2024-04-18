mod hasher;
use std::hash::Hash;
pub use hasher::hash;
use std::borrow::Borrow;


const BSIZE:usize = 8;
const BROW:usize = 8;

#[derive(Debug)]
pub struct BucketList<K,V>{
    seed:u64,
    len:usize,
    buckets: Vec<Vec(K ,V)>>,
}

impl <K:Hash+Eq, V> BucketList<K, V>{
    
}









#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
