use crate::packet::Body;
use crate::packet::Message;

pub struct Node {
    pub messages: Vec<u64>,
    pub node_id: String,
    pub message_id: u64,
}

pub trait Actions {
    fn on_init(&mut self, message: Message) -> Option<Message>;
    fn on_broadcast_received(&mut self, message: Message) -> Option<Message>;
    fn on_read(&mut self, message: Message) -> Option<Message>;
    fn on_topografy(&mut self, message: Message) -> Option<Message>;

    fn get_node_id(&self) -> String;

    fn get_next_msg_id(&mut self) -> u64;
}

impl Actions for Node {
    fn on_init(&mut self, message: Message) -> Option<Message> {
        match message.body {
            Body::Init {
                msg_id,
                node_id,
                node_ids: _,
            } => {
                if self.node_id.is_empty() {
                    self.node_id = String::from(node_id)
                }

                let to_send = Message {
                    src: message.dest,
                    dest: message.src,
                    body: Body::InitOk {
                        in_reply_to: msg_id,
                    },
                };

                return Some(to_send);
            }
            _ => None,
        }
    }

    fn on_broadcast_received(&mut self, message: Message) -> Option<Message> {
        match message.body {
            Body::Broadcast {
                msg_id,
                message: message_value,
            } => {
                self.messages.push(message_value);

                let to_send = Message {
                    src: message.dest,
                    dest: message.src,
                    body: Body::BroadcastOk {
                        msg_id: self.get_next_msg_id(),
                        in_reply_to: msg_id,
                    },
                };

                return Some(to_send);
            }
            _ => None,
        }
    }

    fn on_read(&mut self, message: Message) -> Option<Message> {
        match message.body {
            Body::Read { msg_id } => {
                let to_send = Message {
                    src: message.dest,
                    dest: message.src,
                    body: Body::ReadOk {
                        msg_id: self.get_next_msg_id(),
                        messages: self.messages.clone(),
                        in_reply_to: msg_id,
                    },
                };

                return Some(to_send);
            }
            _ => None,
        }
    }

    fn on_topografy(&mut self, message: Message) -> Option<Message> {
        match message.body {
            Body::Topology { msg_id } => {
                let to_send = Message {
                    src: message.dest,
                    dest: message.src,
                    body: Body::TopologyOk {
                        msg_id: self.get_next_msg_id(),
                        in_reply_to: msg_id,
                    },
                };

                return Some(to_send);
            }
            _ => None,
        }
    }

    fn get_node_id(&self) -> String {
        self.node_id.clone()
    }

    fn get_next_msg_id(&mut self) -> u64 {
        self.message_id += 1;
        self.message_id
    }
}
