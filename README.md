# MiniDHT
A toy implementation of the Kademlia Distributed Hash Table in rust. 

This project should never be used in production and is mostly being used for me to learn Rust. The implementation is also incomplete and APIs will change frequently in the future.


## Todo
 - [x] Replace rustc_serialize with serde
 - [x] Implement larger key size
 - [x] Add an option to use SHA 256 instead of SHA1
 - [x] Proper CLI interface
 - [ ] Accept files for initial peer list
 - [ ] Run benchmarks against other Kademlia implementations
 - [ ] Write unit tests for major features

## Credits
Based on https://github.com/leejunseok/kademlia-rs and http://pdos.csail.mit.edu/~petar/papers/maymounkov-kademlia-lncs.pdf
