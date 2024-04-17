use ldk_node::{Builder, Network};
use std::env;

fn main() {
    // No need to turn this variable off after you running this program
    env::set_var("RUST_BACKTRACE", "full");
    let mut builder = Builder::new();
    builder.set_network(Network::Testnet);
    builder.set_esplora_server("https://blockstream.info/testnet/api".to_string());
    let node = builder.build().unwrap();

    node.start().unwrap();
    println!("NODE_ID: {}", node.node_id());
    node.stop().unwrap();
}
