use crate::driver::Driver;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};

struct Response {
    get: Vec<(String, String)>,
    fallback: (String, String),
}

pub struct Server {
    port: i32,
    listener: TcpListener,
    driver: Driver,
    response: Response,
}

impl Server {
    pub fn new(port: i32) -> Self {
        let listener =
            TcpListener::bind(format!("127.0.0.1:{0}", port)).expect("Failed to bind to port");

        Server {
            port,
            listener,
            driver: Driver::new("public"),
            response: Response { get: Vec::new(), fallback: (String::new(), String::new()) },
        }
    }

    pub fn get(&mut self, uri: &str, path: &str) {
        self.response.get.push((uri.to_string(), path.to_string()));
    }

    pub fn fallback(&mut self, path: &str) {
        self.response.fallback = (String::from("/"), path.to_string());
    }

    pub fn run(&self) {
        println!("Server running on port {}", self.port);

        for stream in self.listener.incoming() {
            match stream {
                Ok(_stream) => self.handle_request(_stream),
                Err(e) => {
                    eprintln!("Failed to accept connection: {}", e);
                }
            }
        }
    }

    fn handle_request(&self, _stream: TcpStream) {
        let buffer = BufReader::new(&_stream);
        let request: Vec<_> = buffer
            .lines()
            .map(|line| line.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        let code = request.get(0).unwrap().split(" ").collect::<Vec<&str>>();
        let uri = code.get(1).unwrap_or(&"/");

        self.handle_response(_stream, code.get(0).unwrap(), uri);
    }

    fn handle_response(&self, mut stream: TcpStream, code: &str, uri: &str) {
        println!("Request received: {0} {1}", code, uri);

        match code {
            "GET" => {
                let fallback = &self.response.fallback;

                let (_, file) = self
                    .response
                    .get
                    .iter()
                    .find(|(_name, _value)| _name == uri)
                    .unwrap_or(&fallback);

                let content = self.driver.get_file(file.as_ref());
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {0}\r\n\r\n{1}",
                    content.len(),
                    content
                );

                stream
                    .write_all(response.as_bytes())
                    .expect("Failed to write to stream");
            }
            _ => {
                let not_found_response = "HTTP/1.1 404 Not Found\r\n\r\n";

                stream
                    .write_all(not_found_response.as_bytes())
                    .expect("Failed to write to stream");
            }
        }
    }
}
