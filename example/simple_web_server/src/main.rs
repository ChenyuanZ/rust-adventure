use std::fs;
use std::io::prelude::*;  // It contains traits to read from and write to the stream.
use std::net::TcpStream;
use std::net::TcpListener;
use std::thread;
use std::time::Duration;

fn main() {
    // The 'bind' function return a new TcpListener instance.
    // We use 'unwrap' to stop the program if errors happen.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // 'incoming' returns an iterator that gives us a sequence of 'TcpStream'.
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

// We made 'stream' mutable because the TcpStream instance keeps track of what data it returns to
// us internally. It might read more data than we asked for and save that data for the next time we ask for data.
fn handle_connection(mut stream: TcpStream) {
    // 'buffer' is used to hold the data that is read in. It is 512 bytes in size.
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    // 'from_utf8_lossy' replace invalid UTF-8 sequence with �.
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // b"" -- byte string syntax
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        // The 'CRLF' sequence can also be written as \r\n, where \r is a carriage return and \n is a line feed.
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
