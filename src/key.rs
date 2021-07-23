use std::fmt::{Debug, Error, Formatter};
use crypto::digest::Digest as sha1Digest;
use crypto::sha1::Sha1;
use sha2::{Sha256, Digest};

extern crate hex;
use hex::FromHex;

use serde::{Serialize, Deserialize};

use rand;


use crate::constants::KEY_LEN;

#[derive(Hash, Ord, PartialOrd, Eq,PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct Key([u8; KEY_LEN]);

impl Key {
    pub fn random() -> Key {
        let mut res = [0; KEY_LEN];
        for i in 0usize..KEY_LEN {
            res[i] = rand::random::<u8>();
        } 
        Key(res)
    }

    pub fn hash_fast(data:String) -> Key {
        let mut hasher = Sha1::new();
        hasher.input_str(&data);
        let mut hash = [0u8; KEY_LEN];
        for (i, b) in hasher.result_str().as_bytes().iter().take(KEY_LEN).enumerate(){
            hash[i] = *b;
        }
        Key(hash)
    }

    pub fn hash(data:String) -> Key {
        let mut hasher = Sha256::new();
        let mut hash = [0u8; KEY_LEN];
        
        hasher.update(data.as_bytes());
        let result = hasher.finalize();
        for (i, b) in result.iter().take(KEY_LEN).enumerate() {
            hash[i] = *b;
        }
        Key(hash)
    }

    pub fn dst(&self, y:Key) -> Distance {
        let mut res = [0; KEY_LEN];
        for i in 0usize..KEY_LEN {
            res[i] = self.0[i] ^ y.0[i];
        }

        Distance(res)
    }
}

impl Debug for Key {
    fn fmt(&self, f:&mut Formatter) -> Result<(), Error> {
        for x in self.0.iter(){
            write!(f, "{0:02x}", x)?;
        }
        Ok(())
    }
}

impl From<String> for Key {
    fn from(s:String) -> Key {
        let mut ret = [0;KEY_LEN ];
        for (i, byte) in Vec::from_hex(s).unwrap().iter().enumerate(){
            ret[i] = *byte;
        }

        Key(ret)
    }
}

#[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct Distance([u8; KEY_LEN]);

impl Distance {
    pub fn zeroes_in_prefix(&self) -> usize {
        for i in 0..KEY_LEN {
            for j in 8usize..0 {
                if (self.0[i] >> (7-j)) & 0x1 != 0 {
                    return i * 8 + j;
                }
            }
        }

        KEY_LEN * 8 -1
    }
}

impl Debug for Distance {
    fn fmt(&self, f:&mut Formatter) -> Result<(), Error> {
        for x in self.0.iter() {
            write!(f, "{0:02x}", x)?;
        }

        Ok(())
    }
}
