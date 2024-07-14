use std::{
    fs,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

struct Request {
    method: String,
    path: String,
    version: String,
    headers: Vec<String>,
    body: Option<Vec<u8>>,
}

impl Request {
    fn new(reader: &mut BufReader<TcpStream>) -> Self {
        let mut request_line = String::new();
        let mut headers: Vec<String> = Vec::new();

        if let Ok(_) = reader.read_line(&mut request_line) {
            let mut header_line = String::new();
            while let Ok(_) = reader.read_line(&mut header_line) {
                if header_line == "\r\n" {
                    break;
                }
                headers.push(header_line.trim().to_string());
                header_line.clear();
            }
        }

        let request_line_data: Vec<&str> = request_line.split(' ').collect();

        let mut request = Request {
            method: request_line_data[0].to_string(),
            path: request_line_data[1].to_string(),
            version: request_line_data[2].to_string(),
            headers,
            body: None,
        };

        let content_length: usize = request.get_header("content-length").parse().unwrap_or(0);

        let mut body = vec![0; content_length];
        reader.read_exact(&mut body).unwrap();

        request.body = Some(body);

        request
    }

    fn get_header(self: &Self, _target_header: &str) -> String {
        let _lower_target_header = _target_header.to_lowercase();
        let target_header = _lower_target_header.trim();
        self.headers
            .iter()
            .find_map(|header| {
                if header.to_lowercase().starts_with(target_header) {
                    Some(
                        header
                            .to_lowercase()
                            .replace(format!("{}:", target_header).as_str(), "")
                            .trim()
                            .to_string(),
                    )
                } else {
                    None
                }
            })
            .unwrap_or_default()
            .to_string()
    }
}

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
            "HTTP/1.1 {}\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n{}",
            self.status, content_length, "text/plain", self.body
        )
    }
}

fn handle_client(mut stream: TcpStream) {
    println!("accepted new connection");

    let mut reader = BufReader::new(stream);

    let request = Request::new(&mut reader);

    // let body = fs::read_to_string("hello.html").unwrap();
    let response;
    if request.path.eq("/") {
        response = Response::new("200 OK", "");
    } else if request.path.eq("/echo") {
        response = Response::new("200 OK", "");
    } else if request.path.starts_with("/echo/") {
        let body = request.path.replace("/echo/", "");
        response = Response::new("200 OK", &body);
    } else {
        response = Response::new("404 Not Found", "");
    }

    let mut inner = reader.into_inner();
    if let Err(e) = inner.write(response.to_string().as_bytes()) {
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
