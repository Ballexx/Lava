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

        let mut header: String = String::from("");

        for line in buffer.lines(){
            let line = line.unwrap();

            println!("{}", line);

            if line == String::from("\r\n"){
                break;
            }
            //Needs fixing here -----------------
            //error finding EOF resulting in infinite header-loop

            header = header + &line + "\r\n";
        }

        request::read_request(header);
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
