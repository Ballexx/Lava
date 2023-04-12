use super::{router::Route, http::request};
use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Read}};

pub struct Server{
    pub host: String,
    pub port: i32,
    pub mount: Vec<Route>
}

impl Server{
    fn bind(&self) -> TcpListener{
        let addr = format!("{}:{}", self.host, self.port);
        let bind = TcpListener::bind(addr).expect("Error binding socket");

        return bind;
    }   

    fn handle_connection(mut stream: &TcpStream){
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

        request::read_request(String::from(string_buffer));
    }       

    pub fn listen(&self){
        let socket = self.bind(); 

        for stream in socket.incoming(){
            match stream {
                Ok(stream) => {
                    Server::handle_connection(&stream);
                }
                Err(e) => { println!("{}", e); }
            }
        }
    }   
}
