use std::{
    io::{Read, Write},
    net::TcpStream,
};

#[derive(Debug)]
pub struct HealthCheck {
    server_host_list: Vec<&'static str>,
    healthy_servers: Vec<&'static str>,
}

impl HealthCheck {
    pub fn new(server_host_list: Vec<&'static str>) -> Self {
        HealthCheck {
            server_host_list,
            healthy_servers: Vec::new(),
        }
    }

    pub fn check(&mut self) {
        for server_host in &self.server_host_list {
            if self._is_server_healthy(server_host) {
                self.healthy_servers.push(server_host);
            }
        }
    }

    pub fn get_healthy_servers(&self) -> &Vec<&'static str> {
        &self.healthy_servers
    }

    fn _is_server_healthy(&self, server_host: &str) -> bool {
        let response = self._send_health_request(server_host);
        return self._parse_health_response(response);
    }

    fn _generate_health_call(&self, server_host: &str) -> String {
        return format!(
            "GET /health HTTP/1.1\r\nHost: {}\r\nAccept: */*\r\n\r\n",
            server_host
        );
    }

    fn _parse_health_response(&self, response: String) -> bool {
        return response.contains("200 OK");
    }

    fn _send_health_request(&self, server_host: &str) -> String {
        let health_call = self._generate_health_call(server_host);
        let mut server_conn = TcpStream::connect(server_host).unwrap();
        server_conn.write_all(health_call.as_bytes()).unwrap();

        let mut response = String::new();
        server_conn.read_to_string(&mut response).unwrap();
        return response;
    }
}
