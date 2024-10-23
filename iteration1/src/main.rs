// use std::{
//     io::{Read, Write},
//     net::TcpListener,
// };
//
// fn main() {
//     let listener = TcpListener::bind("127.0.0.1:80").unwrap();
//     let mut counter = 0;
//     for stream in listener.incoming() {
//         let mut stream = stream.unwrap();
//         counter += 1;
//         let mut buffer = [0; 1024];
//         let bytes_read = stream.read(&mut buffer).unwrap();
//         let request = String::from_utf8(buffer[..bytes_read].to_vec()).unwrap();
//         assert_eq!(
//             request.lines().next().unwrap(),
//             "GET / HTTP/1.1",
//             "can respond only to get request in http 1.1"
//         );
//         let body = counter.to_string();
//         let content_length = body.len();
//         let response = format!(
//             "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {content_length}\r\n\r\n{body}",
//         );
//         stream.write(response.as_bytes()).unwrap();
//         stream.flush().unwrap();
//     }
// }

fn main() {
    println!("cross compiling is fun");
}
