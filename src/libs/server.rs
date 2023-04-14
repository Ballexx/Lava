use super::{router::Route, http::{request::{self, Request}, response::Response}};
use std::{net::{TcpListener, TcpStream}, io::{Read}};

pub struct Server{
    pub host:   String,
    pub port:   i32,
    pub routes:  Vec<Route>
}

impl Server{
    fn bind(&self) -> TcpListener{
        let addr = format!("{}:{}", self.host, self.port);
        let bind = TcpListener::bind(addr).expect("Error binding socket");

        return bind;
    }   

    pub fn erupt(&self){
        let socket = self.bind(); 

        for stream in socket.incoming(){
            match stream {
                Ok(stream) => {
                    let request = Server::read_connection(&stream);
                    self.handle_connection(request)
                }
                Err(e) => { println!("{}", e); }
            }
        }
    }

    fn handle_connection(&self, request: Request){
        for route in &self.routes{
            if route.get_path() as &str != &request.route as &str{
                continue;
            }
            else if route.get_method() as &str != &request.method as &str {
                continue;
            }

            let res = Response{status: 200, body: "test".to_string(), header: "test".to_string()};

            route.get_func()();
        }
    }

    fn read_connection(mut stream: &TcpStream) -> Request{
        let mut buffer: Vec<u8> = Vec::new();
        let mut tempbuffer = [0 as u8; 4096];

        loop {
            match stream.read(&mut tempbuffer) {
                Ok(_) => {
                    for byte in tempbuffer{
                        if byte == 0{
                            break;
                        }
                        buffer.push(byte);
                    }
                    break;
                },
                Err(_) => {
                    println!("Error reading connection");
                }
            }
        }
        let string_buffer = String::from_utf8_lossy(&buffer);

        return request::read_request(String::from(string_buffer));
    }           
}
