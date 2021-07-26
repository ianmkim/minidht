#[cfg(test)]
mod tests {
    use crate::node::*;
    use crate::key::*;
    use crate::routing::*;

    #[test]
    fn create_node_without_bootstrap() {
        let _ = Node::start(String::from("test_net"),
            Key::random(),
            "127.0.0.1:0",
            None);
    }

    #[test]
    fn create_node_with_bootstrap() {
        let to_bootstrap_with = Node::start(String::from("test_net"),
           Key::random(),
           "127.0.0.1:0",
           None);

        let bootstrap = to_bootstrap_with.node_info.clone();
        let opt_bootstrap: Option<NodeInfo> = Some(bootstrap.clone());

        let with_bs = Node::start(String::from("test_net"),
           Key::random(),
            "127.0.0.1:0",
            opt_bootstrap);

        let mut counter = 0;
        for layer_vec in with_bs._return_routing_table()._get_buckets() {
            counter += layer_vec.iter()
                .map(|item| if *item == bootstrap {1} else {0})
                .sum::<i32>();
        }

        assert_eq!(counter, 1);
    }

    #[test]
    fn dense_store_test(){
        let mut last_boostrap:Option<NodeInfo> = None;
        let mut nodes: Vec<Node> = Vec::new();
        for _ in 1..10 {
            let temp_node = Node::start(String::from("test_net"),
                Key::random(),
                "127.0.0.1:0",
                last_boostrap);
            last_boostrap = Some(temp_node.node_info.clone());
            nodes.push(temp_node);
        }

        let test_val = "test value";
        nodes[0].put(String::from("test key"), String::from(test_val));
        let to_drop = nodes.remove(0);
        drop(to_drop);
        let output = nodes[nodes.len()-1].get(String::from("test key")).unwrap();
        assert_eq!(String::from(test_val), output);
    }
}
