use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use crate::driver::Driver;

pub struct Server {
    port: i32,
    listener: TcpListener,
    driver: Driver
}

impl Server {
    pub fn new(port: i32) -> Self {
        let listener =
            TcpListener::bind(format!("127.0.0.1:{0}", port)).expect("Failed to bind to port");

        Server { port, listener, driver: Driver::new("public") }
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

        self.handle_response(_stream);
    }

    fn handle_response(&self, mut stream: TcpStream) {
        let file = self.driver.get_file("index.html".as_ref());

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {0}\r\n\r\n{1}",
            file.len(),
            file
        );

        stream.write_all(response.as_bytes()).expect("Failed to write to stream");
    }
}
