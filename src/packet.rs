use std::io::Write;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Body {
    #[serde(rename = "init")]
    Init {
        msg_id: u64,
        node_id: String,
        node_ids: Vec<String>,
    },
    #[serde(rename = "init_ok")]
    InitOk { in_reply_to: u64 },
    #[serde(rename = "broadcast")]
    Broadcast { msg_id: u64, message: u64 },
    #[serde(rename = "broadcast_ok")]
    BroadcastOk { msg_id: u64, in_reply_to: u64 },
    #[serde(rename = "read")]
    Read { msg_id: u64 },
    #[serde(rename = "read_ok")]
    ReadOk {
        msg_id: u64,
        messages: Vec<u64>,
        in_reply_to: u64,
    },
    #[serde(rename = "topology")]
    Topology { msg_id: u64 },
    #[serde(rename = "topology_ok")]
    TopologyOk { msg_id: u64, in_reply_to: u64 },
    #[serde(rename = "error")]
    Error {
        in_reply_to: u64,
        code: u64,
        text: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub src: String,
    pub dest: String,
    pub body: Body,
}

pub fn write_to_stdout(message: Message) {
    let mut stdout = std::io::stdout();

    let output_json = serde_json::to_string(&message).unwrap();
    writeln!(stdout, "{}", output_json).unwrap();
    stdout.flush().unwrap();
}
