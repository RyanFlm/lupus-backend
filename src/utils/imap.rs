use std::net::TcpStream;

use imap::{self, Session};
use rustls_connector::{
    rustls::{ClientConnection, StreamOwned},
    RustlsConnector,
};
pub struct ConnectOptions {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub starttls: bool,
}

pub fn connect(
    options: ConnectOptions,
) -> Option<Session<StreamOwned<ClientConnection, TcpStream>>> {
    let mut client_builder = imap::ClientBuilder::new(options.host, options.port);

    if options.starttls {
        client_builder.starttls();
    }

    let client_result = client_builder.connect(|domain, tcp| {
        let ssl_conn = RustlsConnector::new_with_native_certs()?;
        Ok(ssl_conn.connect(domain, tcp)?)
    });

    Some(
        client_result
            .expect("Failed to create client")
            .login(options.username, options.password)
            .map_err(|e| e.0)
            .expect("Login failed"),
    )
}
