use std::io::{self, BufRead};

use packet_init::write_to_stdout;
use packet_init::Body;
use packet_init::Message;
use tokio;

mod packet_init;

#[tokio::main]
async fn main() {
    let mut node_id = String::new();

    for line in io::stdin().lock().lines() {
        let input: Message = serde_json::from_str(&line.unwrap()).unwrap();
        match input.body {
            Body::Init {
                msg_id,
                node_id: id,
                ..
            } => {
                // Setting this node id
                node_id = id;

                let output = Message {
                    src: node_id.clone(),
                    dest: input.src,
                    body: Body::InitOk {
                        in_reply_to: msg_id,
                    },
                };

                tokio::spawn(async { write_to_stdout(output) });
            }
            Body::Echo { msg_id, echo } => {
                let output = Message {
                    src: node_id.clone(),
                    dest: input.src,
                    body: Body::EchoOk {
                        msg_id: msg_id,
                        in_reply_to: msg_id,
                        echo: echo,
                    },
                };

                tokio::spawn(async { write_to_stdout(output) });
            }
            Body::Error {
                in_reply_to,
                code,
                text,
            } => {
                eprintln!(
                    "Error received (in_reply_to: {}, code: {}, text: {})",
                    in_reply_to, code, text
                );
            }
            _ => (),
        }
    }
}
