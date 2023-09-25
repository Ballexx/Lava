use std::{
    net::{
        TcpListener, 
        TcpStream
    }, 
    io::{
        Read, 
        Write, SeekFrom
    }, 
    fmt::format
};

use crate::server::route::Route;
use crate::request::req_parse::{
    parse_request_header,
    Request
};

fn read_connection(mut stream: &TcpStream) -> String{
    const BUFFER_SIZE: usize = 4096;
    let mut buffer = [0; BUFFER_SIZE];

    let mut data: Vec<u8> = vec![];

    loop{
        let bytes_read: usize = stream.read(&mut buffer).unwrap();

        data.extend_from_slice(&buffer[..bytes_read]);

        if bytes_read < BUFFER_SIZE{
            break;
        }
    }

    let request_header: String = String::from(
        core::str::from_utf8(&data).unwrap()
    );
    
    return request_header;
}

pub struct Server{
    pub host:      String,
    pub port:      i32,
    pub routes:    Vec<Route>
}

impl Server{
    fn bind(&self) -> TcpListener{
        let addr: String = format!(
            "{}:{}", self.host, self.port
        );

        let bind: TcpListener = TcpListener::bind(
            addr).expect("Error binding socket");

        const VOLCANO_EMOJI: char = '\u{1F30B}';
        println!("\r\n\r\nThe volcano is live! {VOLCANO_EMOJI} {VOLCANO_EMOJI} {VOLCANO_EMOJI}\r\n\r\n");

        return bind;
    }

    pub fn execute_user_func(&self, request: Request){
        for route in &self.routes{
            if request.path != route.path{
                continue;
            }
            else if request.method != route.method{
                continue;
            }
            route.get_func()()
        }
    }

    pub fn erupt(&self){
        let listener: TcpListener = self.bind();
        
        loop {
            let mut stream: TcpStream = listener.accept().unwrap().0;
            let data_read: String = read_connection(&stream);

            let request: Request = parse_request_header(
                data_read
            );
            self.execute_user_func(request);

            let response_header: String = format!(
                "HTTP/1.1 {}\r\n{}\r\n\r\n{}", 200, "", ""
            );
    
            stream.write_all(response_header.as_bytes());
            stream.flush();
        }
    }
}
