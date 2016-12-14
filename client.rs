use std::io::prelude::*;
use std::net::TcpStream;
use rustc_serialize::json::decode;
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::fs::File;

use rori_utils::data::RoriData;

// TODO sslstream
pub struct RoriClient {
    address: String,
}

#[derive(Clone, RustcDecodable, RustcEncodable, Default, PartialEq, Debug)]
pub struct ConfigServer {
    pub ip: Option<String>,
    pub port: Option<String>,
}

impl RoriClient {
    pub fn parse_config(data: String) -> String {
        let params: ConfigServer = decode(&data[..])
            .map_err(|_| {
                Error::new(ErrorKind::InvalidInput,
                           "Failed to decode configuration file.")
            })
            .unwrap();

        format!("{}:{}",
                &params.ip.unwrap_or(String::from("")),
                &params.port.unwrap_or(String::from("")))
    }

    pub fn new<P: AsRef<Path>>(config: P) -> RoriClient {
        // Configure from file
        let mut file = File::open(config)
            .ok()
            .expect("Config file not found");
        let mut data = String::new();
        file.read_to_string(&mut data)
            .ok()
            .expect("failed to read!");
        let address = RoriClient::parse_config(data);
        if address == ":" {
            println!("Empty config for the connection to the server");
        }
        RoriClient { address: address }
    }

    pub fn send_to_rori(&mut self, author: &str, content: &str) {
        let data_to_send = RoriData::new(String::from(author),
                                         String::from(content),
                                         String::from("irc_entry_module"),
                                         String::from("text"));

        let mut stream = TcpStream::connect(&*self.address).unwrap();
        let _ = stream.write(data_to_send.to_string().as_bytes());
    }
}