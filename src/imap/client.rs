use std::{
    io::{self, BufRead, BufReader, Write},
    net::TcpStream,
    str,
};

use super::error::Result as IResult;
use super::types::{Command, ServerResponse, State, TagRepr};
use log::{debug, info};
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
    pub fn new(domain: &str, port: &str) -> Result<Self, io::Error> {
        let connector = SslConnector::builder(SslMethod::tls_client())
            .unwrap()
            .build();
        let stream = TcpStream::connect(format!("{}:{}", domain, port))?;
        let stream = connector.connect(domain, stream).unwrap();
        let mut client = Self {
            stream,
            tag: TagRepr::new(),
            state: State::NotAuthenticated,
        };
        let _ = client.receive(); // Sends an Ok message at the beginning of communication
        Ok(client)
    }

    /// Only call while in State::NotAuthenticated
    pub fn login(&mut self, user: &str, pass: &str) -> IResult<()> {
        self.send(Command::Login(user, pass))?;
        let response = self.receive();
        Ok(())
    }

    fn send(&mut self, command: Command) -> IResult<()> {
        // TODO: Remove once tested enough
        command.check(&self.state)?;
        let msg = format!("{} {}", self.tag, command);

        info!("{}", msg);
        self.stream.write(msg.as_bytes()).unwrap();
        self.tag.inc();
        Ok(())
    }

    pub fn receive(&mut self) -> ServerResponse {
        let mut reader = BufReader::new(&mut self.stream);
        let mut buf = Vec::new();
        reader.read_until(b'\r', &mut buf).unwrap();
        let s = String::from_utf8(buf).unwrap();
        debug!("{}", s);
        ServerResponse::from(s)
    }
}
