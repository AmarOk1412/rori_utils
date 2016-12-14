use rustc_serialize::json::decode;
use std::io::{Error, ErrorKind};

#[derive(Clone, RustcDecodable, RustcEncodable, Default, PartialEq, Debug)]
pub struct RoriData {
    pub author: String,
    pub content: String,
    pub client: String,
    pub datatype: String,
}

#[allow(dead_code)]
impl RoriData {
    pub fn new(author: String, content: String, client: String, datatype: String) -> RoriData {
        RoriData {
            author: author.replace("\"", "\\\""),
            content: content.replace("\"", "\\\""),
            client: client.replace("\"", "\\\""),
            datatype: datatype,
        }
    }

    pub fn from_json(json: String) -> RoriData {
        let params: RoriData = decode(&json[..])
            .map_err(|_| Error::new(ErrorKind::InvalidInput, "Failed to decode json."))
            .unwrap();
        params
    }

    pub fn to_string(&self) -> String {
        format!("{{
            \"author\":\"{}\",
            \"content\":\"{}\",
            \"client\":\"{}\",
            \"type\":\"{}\"
        }}",
                self.author,
                self.content,
                self.client,
                self.datatype)
    }
}
