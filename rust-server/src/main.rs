use std::env;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::process;

fn exit_early(msg: String) -> i16 {
    println!("{}", msg);
    process::exit(1);
}

fn main() {
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

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();
    let resp: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[..]);
    let body: String = resp.chars().rev().collect();
    stream.write(body.as_bytes()).unwrap();
}
