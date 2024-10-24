use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() {
    println!("starting");
    let listener = TcpListener::bind("0.0.0.0:80").unwrap();
    println!("tcp listener binded");
    let mut counter = 0;
    for stream in listener.incoming() {
        println!("incoming stream");
        let mut stream = stream.unwrap();
        println!("received");
        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer).unwrap();
        println!("read to buffer");
        let request = String::from_utf8(buffer[..bytes_read].to_vec()).unwrap();
        println!("build string from buffer");
        let request_line = request.lines().next().unwrap();
        println!("{request_line}");
        match request_line {
            "GET / HTTP/1.1" => {
                counter += 1;
                let body = counter.to_string();
                let content_length = body.len();
                let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {content_length}\r\n\r\n{body}");
                stream.write(response.as_bytes()).unwrap();
                println!("wrote to buffer");
                stream.flush().unwrap();
                println!("flushed");
            }
            "GET /favicon.ico HTTP/1.1" => {
                let buffer = include_bytes!("../favicon.ico").to_vec();
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: image/x-icon\r\nContent-Length: {}\r\n\r\n",
                    buffer.len()
                );
                stream.write(response.as_bytes()).unwrap();
                stream.write(&buffer).unwrap();
                stream.flush().unwrap();
            }
            _ => {
                let response = "HTTP/1.1 500 Internal Server Error\r\n\r\n";
                stream.write_all(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
        }
    }
}
