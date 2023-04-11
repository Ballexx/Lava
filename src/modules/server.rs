use super::{router::Route, http::request};
use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead}};

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

    fn handle_connection(stream: TcpStream){
        let buffer = BufReader::new(stream);

        for line in buffer.lines(){
            request::request_reader(line.unwrap());
        }
    }   

    pub fn listen(&self){
        let socket = self.bind(); 

        for stream in socket.incoming(){
            match stream {
                Ok(stream) => {
                    Server::handle_connection(stream);
                }
                Err(e) => { println!("{}", e); }
            }
        }
    }   
}
