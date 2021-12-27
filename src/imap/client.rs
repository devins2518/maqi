use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    str,
};

use super::error::ImapResult;
use super::types::{Command, ServerResponse, State, TagRepr};
use log::info;
use openssl::ssl::{SslConnector, SslMethod, SslStream};

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
    stream: SslStream<TcpStream>,
    tag: TagRepr,
    state: State,
}

impl ImapClient {
    pub fn new(domain: &str, port: &str) -> ImapResult<Self> {
        let connector = SslConnector::builder(SslMethod::tls_client())?.build();
        let stream = TcpStream::connect(format!("{}:{}", domain, port))?;
        let stream = connector.connect(domain, stream)?;
        let mut client = Self {
            stream,
            tag: TagRepr::new(),
            state: State::NotAuthenticated,
        };
        let _ = client.receive(); // Sends an Ok message at the beginning of communication
        Ok(client)
    }

    /// Only call while in State::NotAuthenticated
    pub fn login(&mut self, user: &str, pass: &str) -> ImapResult<()> {
        self.send(Command::Login(user, pass))?;
        let response = self.receive()?;
        if let Some(e) = response.is_err() {
            Err(e)
        } else {
            Ok(())
        }
    }

    fn send(&mut self, command: Command) -> ImapResult<()> {
        // TODO: Remove once tested enough
        command.check(&self.state)?;
        let msg = format!("{} {}", self.tag, command);

        info!("Sent: {}", msg);
        self.stream.write(msg.as_bytes()).unwrap();
        self.stream.write(b"\r\n").unwrap();
        self.stream.flush()?;
        self.tag.inc();
        Ok(())
    }

    pub fn receive(&mut self) -> ImapResult<ServerResponse> {
        let mut reader = BufReader::new(&mut self.stream);
        let mut buf = Vec::new();
        reader.read_until(b'\r', &mut buf)?;
        let s = String::from_utf8(buf).unwrap();
        info!("Received: {}", s);
        Ok(ServerResponse::from(s))
    }
}

impl Drop for ImapClient {
    fn drop(&mut self) {
        // Can fail if connection is lost before dropping self
        self.send(Command::Logout).unwrap_or(());
        drop(self);
    }
}
