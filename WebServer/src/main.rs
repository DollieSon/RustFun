use core::prelude;
use std::{
    env::consts,
    fs::read_to_string,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    ptr::read,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_req: Vec<_> = buf_reader
        .lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    let status_line = "HTTP/1.1 200 OK";
    let contents = read_to_string("response.html").unwrap();
    let len = contents.len();
    let response = format!("{status_line}\r\nContent_Len:{len}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
    // println!("Request: {http_req:#?}");
}
