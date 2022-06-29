use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::process;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap_or_else(|e| {
        println!("Error: {}", e);
        process::exit(1);
    });

    for stream in listener.incoming() {
        let stream = stream.unwrap_or_else(|e| {
            println!("Error: {}", e);
            process::exit(1);
        });

        if let Err(e) = handle_connection(stream) {
            println!("{})", e);
        };
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename)?;

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}
