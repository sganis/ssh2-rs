#![deny(warnings)]

extern crate ssh2;
extern crate tempfile;

use std::env;
use std::net::TcpStream;

mod agent;
mod channel;
mod knownhosts;
mod session;
mod sftp;


pub fn test_addr() -> String {
    let host = env::var("TEST_HOST")
        .unwrap_or("127.0.0.1".to_string());
    let port = env::var("TEST_PORT")
        .map(|s| s.parse().unwrap())
        .unwrap_or(22);
    let addr = format!("{}:{}", host, port);
    addr
}

pub fn socket() -> TcpStream {
    TcpStream::connect(&test_addr()).unwrap()
}

pub fn authed_session() -> ssh2::Session {
    let user = env::var("USER").unwrap();
    let socket = socket();
    let mut sess = ssh2::Session::new().unwrap();
    sess.set_tcp_stream(socket);
    sess.handshake().unwrap();
    assert!(!sess.authenticated());

    {
        let mut agent = sess.agent().unwrap();
        agent.connect().unwrap();
        agent.list_identities().unwrap();
        let identities = agent.identities().unwrap();
        let identity = &identities[0];
        agent.userauth(&user, &identity).unwrap();
    }
    assert!(sess.authenticated());
    sess
}
