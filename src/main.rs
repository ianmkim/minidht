extern crate env_logger;

use std::io;

mod node;
mod key;
mod rpc;
mod routing;
mod constants;

use crate::key::*;
use crate::node::*;
use crate::routing::*;

fn interactive() {
    let input = io::stdin();
    let mut buffer = String::new();
    println!("Enter a <IP>:<PORT> <KEY> or leave blank");
    input.read_line(&mut buffer).unwrap();
    let params = buffer.trim_end().split(' ').collect::<Vec<_>>();
    let bootstrap = if params.len() < 2 {
        None
    } else {
        Some(NodeInfo {
            id: Key::from(String::from(params[1])),
            addr: String::from(params[0]),
            net_id: String::from("test_net"),
        })
    };


    let handle = Node::start(String::from("test_net"),
        Key::random(),
        "127.0.0.1:0",
        bootstrap);
    let mut dummy_info = NodeInfo {
        net_id: String::from("test_net"),
        addr: String::from("asdfasdg"),
        id: Key::random(),
    };

    loop {
        let mut buffer = String::new();
        if input.read_line(&mut buffer).is_err() {
            break;
        }
        let args = buffer.trim_end().split(' ').collect::<Vec<_>>();
        match args[0].as_ref() {
            "p" => {
                dummy_info.addr = String::from(args[1]);
                dummy_info.id = Key::from(String::from(args[2]));
                println!("{:?}", handle.ping(dummy_info.clone()));
            }
            "s" => {
                dummy_info.addr = String::from(args[1]);
                dummy_info.id = Key::from(String::from(args[2]));
                println!("{:?}", handle.store(dummy_info.clone(), String::from(args[3]), String::from(args[4])));
            }
            "fn" => {
                dummy_info.addr = String::from(args[1]);
                dummy_info.id = Key::from(String::from(args[2]));
                println!("{:?}", handle.find_node(dummy_info.clone(), Key::from(String::from(args[3]))));
            }
            "fv" => {
                dummy_info.addr = String::from(args[1]);
                dummy_info.id = Key::from(String::from(args[2]));
                println!("{:?}", handle.find_value(dummy_info.clone(), String::from(args[3])));
            }
            "ln" => {
                println!("{:?}", handle.lookup_nodes(Key::from(String::from(args[1]))));
            }
            "lv" => {
                println!("{:?}", handle.lookup_values(String::from(args[1])));
            }
            "put" => {
                println!("{:?}", handle.put(String::from(args[1]), String::from(args[2])));
            }
            "get" => {
                println!("{:?}", handle.get(String::from(args[1])));
            }
            _ => {
                println!("no match");
            }
        }
    }
}

#[allow(deprecated)]
fn main() {
    env_logger::init();
    interactive();
}
