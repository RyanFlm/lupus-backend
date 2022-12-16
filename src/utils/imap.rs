use std::io::{Read, Write};

use imap::{self, ClientBuilder, Error, Session};
use rustls_connector::RustlsConnector;

pub struct ConnectOptions {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub starttls: bool,
}

pub fn connect(options: ConnectOptions) -> Result<Session<impl Read + Write>, String> {
    let mut client_builder = ClientBuilder::new(options.host, options.port);

    // Enable STARTTLS if option is set
    if options.starttls {
        client_builder.starttls();
    }

    // Connect client
    let client_result = client_builder.connect(|domain, tcp| {
        let connector_result = RustlsConnector::new_with_native_certs();
        let connector = match connector_result {
            Ok(c) => c,
            Err(e) => return Err(Error::Io(e)),
        };

        let connection_result = connector.connect(domain, tcp);
        match connection_result {
            Ok(c) => Ok(c),
            Err(e) => Err(Error::RustlsHandshake(e)),
        }
    });

    // Login to get active Session
    let client = match client_result {
        Ok(c) => c,
        Err(e) => return Err(format!("Faild to create client {:?}", e)),
    };

    let session_result = client
        .login(options.username, options.password)
        .map_err(|e| e.0);

    match session_result {
        Ok(s) => Ok(s),
        Err(_) => Err(String::from("Failed to login")),
    }
}
