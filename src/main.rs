use std::{
    io::Write,
    net::{TcpListener, TcpStream},
    thread,
};

struct Response {
    status: String,
    body: String,
}

impl Response {
    fn new(status: &str, body: &str) -> Self {
        Self {
            status: status.to_string(),
            body: body.to_string(),
        }
    }

    fn to_string(&self) -> String {
        let content_length = self.body.len();
        format!(
            "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
            self.status, content_length, self.body
        )
    }
}

fn handle_client(mut stream: TcpStream) {
    println!("accepted new connection");

    let response = Response::new("200 OK", "Hello, client!");

    if let Err(e) = stream.write(response.to_string().as_bytes()) {
        eprintln!("Failed to write to stream: {}", e);
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
