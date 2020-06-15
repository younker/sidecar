use prost::Message;
use std::env;
use std::io::Cursor;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::process;

fn exit_early(msg: String) -> i16 {
    println!("{}", msg);
    process::exit(1);
}

fn set_ctrlc_handler() {
    ctrlc::set_handler(move || {
        println!("Exiting...");
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
}

fn main() {
    set_ctrlc_handler();

    let args: Vec<String> = env::args().collect();
    let port_arg = &args[1];
    let port = match port_arg.parse::<i16>() {
        Ok(n) => n,
        Err(_e) => exit_early(format!("Invalid port provided. {}", port_arg)),
    };

    let address: String = String::from(format!("127.0.0.1:{}", port));
    let tcp_conn = TcpListener::bind(address);
    let listener = match tcp_conn {
        Ok(l) => l,
        Err(e) => panic!(e),
    };
    println!("Server starting on port {}", port);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

// Include the `items` module, which is generated from items.proto.
pub mod items {
    include!(concat!(env!("OUT_DIR"), "/snazzy.items.rs"));
}

pub fn create_large_shirt(color: String) -> items::Shirt {
    let mut shirt = items::Shirt::default();
    shirt.color = color;
    shirt.set_size(items::shirt::Size::Large);
    shirt
}

pub fn serialize_shirt(shirt: &items::Shirt) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(shirt.encoded_len());
    // Unwrap is safe, since we have reserved sufficient capacity in the vector.
    shirt.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize_shirt(buf: &[u8]) -> Result<items::Shirt, prost::DecodeError> {
    items::Shirt::decode(&mut Cursor::new(buf))
}

fn handle_connection(mut stream: TcpStream) {
    // shorthand to initialize all 512 bytes as 0
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let req_body: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[..]);
    let shirt: items::Shirt = create_large_shirt(req_body.to_string());
    let body = serialize_shirt(&shirt);
    stream.write(&body).unwrap();
}
