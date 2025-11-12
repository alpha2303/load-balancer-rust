use std::borrow::Cow;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use super::thread_pool::ThreadPool;

struct StreamHandler {
    stream: TcpStream,
}

impl StreamHandler {
    fn new(stream: TcpStream) -> Self {
        StreamHandler { stream }
    }

    fn handle_connection(&mut self) {
        let mut buffer = [0; 1024];
        match self.stream.read(&mut buffer) {
            Ok(n) => {
                let request_data: Cow<'_, str> = String::from_utf8_lossy(&buffer[..n]);
                println!(
                    "Received request from {} -\n{}",
                    self.stream.peer_addr().unwrap(),
                    request_data
                );

                let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", request_data);
                self.stream.write_all(response.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

pub struct LoadBalancer {
    listener: TcpListener,
    thread_pool: ThreadPool,
}

impl LoadBalancer {
    pub fn new(address: &str) -> Result<Self, std::io::Error> {
        let listener = TcpListener::bind(address).unwrap();
        let thread_pool: ThreadPool = ThreadPool::new(4).unwrap();

        Ok(LoadBalancer {
            listener,
            thread_pool,
        })
    }

    pub fn start(&self) -> Result<(), std::io::Error> {
        println!(
            "Starting load balancer on {}",
            self.listener.local_addr().unwrap()
        );

        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    self.thread_pool.execute(|| {
                        let mut stream_handler = StreamHandler::new(stream);
                        stream_handler.handle_connection();
                    });
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
        Ok(())
    }
}
