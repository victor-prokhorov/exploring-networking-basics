use libc::{
    accept, bind, c_void, listen, read, sockaddr_in, socket, write, AF_INET, INADDR_ANY,
    SOCK_STREAM,
};
use std::{mem, str};

// `man 2 listen`
//
// NOTES
//        To accept connections, the following steps are performed:
//
//            1.  A socket is created with socket(2).
//
//            2.  The socket is bound to a local address using bind(2), so that other sockets may be connect(2)ed to it.
//
//            3.  A willingness to accept incoming connections and a queue limit for incoming connections are specified with listen().
//
//            4.  Connections are accepted with accept(2).

fn main() {
    // `man 2 socket`
    // creates an endpoint for communication and returns a file descriptor that refers to that endpoint
    let sockfd = unsafe { socket(AF_INET, SOCK_STREAM, 0) };
    // On success, a file descriptor for the new socket is returned
    if sockfd < 0 {
        eprintln!("error creating socket");
        return;
    }
    // `man 2 bind`
    let port: u16 = 80;
    let server_address = sockaddr_in {
        sin_family: AF_INET as u16,
        sin_port: port.to_be(), // expected big endian
        sin_addr: libc::in_addr { s_addr: INADDR_ANY },
        sin_zero: [0; 8], // padding
    };
    let bind_result = unsafe {
        bind(
            sockfd,
            // &address as *const _ as *const libc::sockaddr,
            // https://users.rust-lang.org/t/how-do-we-abstract-linux-bind-from-rust-libc/85054/4
            (&server_address as *const sockaddr_in).cast(),
            mem::size_of::<sockaddr_in>() as u32,
        )
    };
    // On success, zero is returned.  On error, -1 is returned, and errno is set appropriately.
    if bind_result < 0 {
        eprintln!("error binding socket to address");
        return;
    }
    // `man 2 listen`
    // `listen` marks the socket as passive, allowing it to accept connections.
    let backlog = 1;
    // The `backlog` argument defines the maximum length to which the queue of pending connections for sockfd may grow.
    let listen_result = unsafe { listen(sockfd, backlog) };
    if listen_result < 0 {
        eprintln!("error listening on socket");
        return;
    }
    println!("server is listening on port 80");
    let mut counter = 0;
    loop {
        // `man 2 listen`
        // It extracts the first connection request on the queue of pending connections
        // for the listening socket, sockfd, creates a new connected socket, and returns a new file descriptor
        // referring to that socket. The newly created socket is not in the listening state.
        let mut client_address: sockaddr_in = unsafe { mem::zeroed() };
        // i guess mutability of length it's more about unique owner
        let mut client_len = mem::size_of::<sockaddr_in>() as u32;
        let client_socket = unsafe {
            accept(
                sockfd,
                (&mut client_address as *mut sockaddr_in).cast(),
                &mut client_len,
            )
        };
        if client_socket < 0 {
            eprintln!("error accepting connection");
            continue;
        }
        let client_port = u16::from_be(client_address.sin_port);
        let client_ip = client_address.sin_addr.s_addr.to_be();
        let ip_bytes = client_ip.to_be_bytes();
        println!(
            "accepted connection from {}.{}.{}.{}:{}",
            ip_bytes[0], ip_bytes[1], ip_bytes[2], ip_bytes[3], client_port
        );
        // `man 2 read`
        let mut buffer = [0; 1024];
        let bytes_read =
            unsafe { read(client_socket, buffer.as_mut_ptr() as *mut _, buffer.len()) };
        if bytes_read < 0 {
            eprintln!("error reading from client socket");
            continue;
        }
        let request = str::from_utf8(&buffer[..bytes_read as usize]).unwrap();
        let request_line = request.lines().next().unwrap();
        println!("received request line: {}", request_line);
        let response_bytes = match request_line {
            "GET / HTTP/1.1" => {
                counter += 1;
                let body = counter.to_string();
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    body.len(),
                    body
                )
                .into_bytes()
            }
            "GET /favicon.ico HTTP/1.1" => {
                let icon_bytes = include_bytes!("../favicon.ico");
                let mut response_bytes = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: image/x-icon\r\nContent-Length: {}\r\n\r\n",
                    icon_bytes.len()
                )
                .into_bytes();
                response_bytes.extend_from_slice(icon_bytes);
                response_bytes
            }
            _ => "HTTP/1.1 404 Not Found\r\n\r\n".bytes().collect(),
        };
        // `man 2 write`
        unsafe {
            write(
                client_socket,
                response_bytes.as_ptr() as *const c_void,
                response_bytes.len(),
            );
        }
        // `man 2 close`
        unsafe { libc::close(client_socket) };
    }
}
