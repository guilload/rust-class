//
// zhttpto.rs
//
// Starting code for PS1
// Running on Rust 0.9
//
// Note that this code has serious security risks!  You should not run it
// on any system with access to sensitive files.
//
// University of Virginia - cs4414 Spring 2014
// Weilin Xu and David Evans
// Version 0.3

use std::io::fs::PathExtensions;
use std::io::{Acceptor, Listener};
use std::io::{File, TcpListener, TcpStream};
use std::str;


fn extract_uri(request: &str) -> &str {
    let mut spaces: Vec<uint> = Vec::with_capacity(2);

    for (i, c) in request.chars().enumerate() {
        if c == ' ' {
            spaces.push(i)
        }

        if spaces.len() == 2 {
            return request.slice(spaces[0] + 2, spaces[1]);
        }
    }

    fail!("Bad request!");
}

fn respond(mut stream: TcpStream, code: int, body: &str) {
    let status = match code {
        200 => "200 OK",
        403 => "403 Forbidden",
        404 => "404 Not Found",
        500 => "500 Internal Server Error",
        _ => fail!(format!("Unknown status code {}", code)),
    };

    let response = format!("HTTP/1.1 {}\r\n\
                            Content-Type: text/html; charset=UTF-8\r\n\r\n\
                            {}\r\n",
                            status, body);
    stream.write(response.as_bytes());
    println!("Connection terminated.");
}

fn handle(mut stream: TcpStream) {

    match stream.peer_name() {
        Ok(peer_name) => println!("Received connection from: {}", peer_name),
        Err(e) => fail!(e)
    }

    let mut buffer = [0, ..500];
    stream.read(buffer);

    let request = str::from_utf8(buffer);
    println!("Received request:\n{}", request);

    let mut uri = match request {
        None => fail!("Bad request!"),
        Some(x) => extract_uri(x),
    };

    if uri == "" {
        uri = "index.html"
    }

    if !uri.ends_with(".html") {
        respond(stream, 403, "");
        return
    }

    let path = Path::new(uri);

    if !path.exists() || !path.is_file() {
        respond(stream, 404, "");
        return
    }

    let mut file = File::open(&path);

    let body = match file.read_to_string() {
        Ok(x) => x,
        Err(e) => fail!(e)
    };

    respond(stream, 200, body.as_slice());
}

fn main() {
    let mut counter = 0i;

    let listener = TcpListener::bind("127.0.0.1", 4414);
    let mut acceptor = listener.listen();
    println!("Listening on 127.0.0.1:4414...");

    for stream in acceptor.incoming() {
        counter += 1;
        println!("So far, the server has handled {} requests.", counter)

        // Spawn a task to handle the connection
        match stream {
            Ok(stream) => spawn(proc() { handle(stream) }),
            Err(e) => { fail!(e) }
        }
    }
}
