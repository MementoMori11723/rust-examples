use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:11500").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    stream.write(get_response(buffer).as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn get_response(buffer: [u8; 1024]) -> String {
    let (filename, status_line) = get_content(buffer);
    let content = fs::read_to_string(filename).unwrap_or_else(|_| {
        fs::read_to_string("pages/500.html").unwrap_or_else(|_| "Internal Server Error".to_string())
    });
    format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        content.len(),
        content
    )
}

fn get_content(buffer: [u8; 1024]) -> (String, String) {
    let request = String::from_utf8_lossy(&buffer);
    match request.lines().next() {
        Some(line) => find_route(line),
        _ => bad_response(),
    }
}

fn find_route(line: &str) -> (String, String) {
    let parts: Vec<&str> = line.split_whitespace().collect();
    match parts.as_slice() {
        [method, path, _] if *method == "GET" => {
            let (f, s) = match *path {
                "/" => ("pages/index.html", "200 OK"),
                "/about" => ("pages/about.html", "200 OK"),
                _ => ("pages/404.html", "404 NOT FOUND"),
            };
            (f.to_string(), s.to_string())
        }
        _ => bad_response(),
    }
}

fn bad_response() -> (String, String) {
    ("pages/400.html".to_string(), "400 BAD REQUEST".to_string())
}
