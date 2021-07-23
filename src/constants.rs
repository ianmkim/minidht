// modified key size to 256 bits from the original 160 bits
pub const KEY_LEN:usize = 32;
pub const N_BUCKETS:usize = KEY_LEN * 8;
pub const K_PARAM: usize = 8;
pub const A_PARAM: usize = 3;
pub const MESSAGE_LEN: usize = 8196;
pub const TIMEOUT:u64 = 5000;

