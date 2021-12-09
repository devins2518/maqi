use std::{
    io::{self, Write},
    net::TcpStream,
    str,
};

use super::types::{Command, ServerResponse, State, TagRepr};

/// TODO:
/// From RFC 9051:
/// [ ] Manipulation of remote mailboxes
/// [ ] Local mailbox synchronization
/// [ ] Create, delete, rename mailboxes
/// [ ] Check for new messages
/// [ ] Remove messages permanently
/// [ ] Set and clear message flags
/// [ ] Parsing per RFC 5322, 2045, 2231
/// [ ] Selective fetching of message attrs, texts, and portions

pub struct ImapClient {
    stream: TcpStream,
    tag: TagRepr,
    state: State,
}

impl ImapClient {
    pub fn new(addr: &str) -> Result<Self, io::Error> {
        Ok(Self {
            stream: TcpStream::connect(addr)?,
            tag: TagRepr::new(),
            state: State::NotAuthenticated,
        })
    }

    pub fn init(&mut self, _user: &str, _pass: &str) {
        todo!("TLS stuff");
    }

    pub fn send(&mut self, command: Command, body: &str) {
        self.stream
            .write(format!("{} {} {}", self.tag, command, body).as_bytes())
            .unwrap();
        self.tag.inc()
    }

    pub fn receive(&mut self) -> ServerResponse {
        todo!()
    }
}
