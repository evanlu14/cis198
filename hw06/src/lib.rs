extern crate rustc_serialize;
extern crate hyper;

use std::io::Read;
use rustc_serialize::json;
use rustc_serialize::{Encodable, Encoder};

use hyper::status::StatusCode;

pub const SERVER_ADDR: &'static str = "127.0.0.1:1980";
pub const BOT_ADDR: &'static str = "127.0.0.1:1981";
pub const HTML_ADDR: &'static str = "http://127.0.0.1:1980";

pub const HTML_DATA: &'static str = "data/index.html";
pub const HTML_HEADER: &'static str = "html/header.html";
pub const HTML_FOOTER: &'static str = "html/footer.html";


#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Message {
    pub user: String,
    pub text: String,
}

impl Message {
    pub fn new(user: String, text: String) -> Message {
        Message {
            text: text,
            user: user,
        }
    }

    pub fn to_string(&self) -> String {
        json::encode(&self).unwrap()
    }
}

pub struct UserClient {
    username: String,
    server_addr: String,
    client: hyper::Client,
}

impl UserClient {
    pub fn new(username: String, server_addr: String) -> UserClient {
        UserClient {
            username: username,
            server_addr: server_addr,
            client: hyper::Client::new(),
        }
    }

    // TODO: Implement send_msg
    pub fn send_msg(&self, content: String) -> hyper::Result<(StatusCode, String)> {
        let m = Message {
            user: self.username.clone(),
            text: content
        };
        let mut response = try!(self.client.post(&self.server_addr).body(&m.to_string()).send());
        let mut buf = String::new();
        response.read_to_string(&mut buf).unwrap();
        Ok((response.status, buf))
    }

    pub fn get_content(&self) -> hyper::Result<(StatusCode, String)> {
        let mut response = try!(self.client.get(&self.server_addr).send());
        let mut buf = String::new();
        response.read_to_string(&mut buf).unwrap();
        Ok((response.status, buf))
    }
}

#[cfg(test)]
mod test {
    use super::Message;
    #[test]
    fn test_message_tostring() {
        let m = Message {
            user : "zc".to_string(),
            text : "sd".to_string()
        };
        println!("{}", m.to_string());
    }
}