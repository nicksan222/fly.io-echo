use std::io::{self, BufRead};

use node::Actions;
use node::Node;
use packet::write_to_stdout;
use packet::Body;
use packet::Message;
use tokio;

mod node;
mod packet;

#[tokio::main]
async fn main() {
    let mut node: Node = Node {
        messages: vec![],
        node_id: String::new(),
        message_id: 0,
    };

    for line in io::stdin().lock().lines() {
        let input: Message = serde_json::from_str(&line.unwrap()).unwrap();
        match input.body {
            Body::Init { .. } => {
                let to_send = node.on_init(input).unwrap();

                write_to_stdout(to_send);
            }
            Body::Broadcast {
                msg_id: _,
                message: _,
            } => {
                let to_send = node.on_broadcast_received(input).unwrap();

                write_to_stdout(to_send)
            }
            Body::Read { msg_id: _ } => {
                let to_send = node.on_read(input).unwrap();

                write_to_stdout(to_send);
            }
            Body::Topology { msg_id: _ } => {
                let to_send = node.on_topografy(input).unwrap();

                write_to_stdout(to_send);
            }
            _ => (),
        }
    }
}
