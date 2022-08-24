use std::io::{Read, Write};

use imap::{self, Session};
use rustls_connector::RustlsConnector;

pub struct ConnectOptions {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub starttls: bool,
}

pub fn connect(options: ConnectOptions) -> Result<Session<impl Read + Write>, String> {
    let mut client_builder = imap::ClientBuilder::new(options.host, options.port);

    // Enable STARTTLS if option is set
    if options.starttls {
        client_builder.starttls();
    }

    // Connect client
    let client_result = client_builder.connect(|domain, tcp| {
        let connector = RustlsConnector::new_with_native_certs().unwrap();
        let connection = connector.connect(domain, tcp).unwrap();
        Ok(connection)
    });

    // Login to get active Session
    let client = match client_result {
        Ok(c) => c,
        Err(_) => return Err(String::from("Failed to create client")),
    };

    let session_result = client.login(options.username, options.password).map_err(|e| e.0);

    match session_result {
        Ok(s) => Ok(s),
        Err(_) => Err(String::from("Failed to login")),
    }
}
