use std::io::Write;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
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
    #[serde(rename = "echo")]
    Echo { msg_id: u64, echo: String },
    #[serde(rename = "echo_ok")]
    EchoOk {
        msg_id: u64,
        in_reply_to: u64,
        echo: String,
    },
    #[serde(rename = "error")]
    Error {
        in_reply_to: u64,
        code: u64,
        text: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
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
