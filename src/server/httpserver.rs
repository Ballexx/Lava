use std::{
    net::{
        TcpListener, 
        TcpStream
    }, 
    io::{
        Read, 
        Write, SeekFrom
    }, 
    fmt::format, string
};

use crate::{
    request::req_parse::{
        parse_request_header,
        Request
    },
    server::route::Route,
    response::res::Response
};

pub struct Server{
    pub host:      &'static str,
    pub port:      i32,
    pub routes:    Vec<Route>
}

impl Server{
    fn bind(&self) -> TcpListener{
        let addr: String = format!(
            "{}:{}", self.host, self.port
        );

        let bind: TcpListener = TcpListener::bind(
            addr
        ).expect(
            "Error binding socket"
        );

        const VOLCANO_EMOJI: char = '\u{1F30B}';
        println!("\r\n\r\nThe volcano is live on port {}! {VOLCANO_EMOJI} {VOLCANO_EMOJI} {VOLCANO_EMOJI}\r\n\r\n", self.port);

        return bind;
    }

    pub fn erupt(&self){
        let listener: TcpListener = self.bind();
        
        loop {
            let mut stream: TcpStream = listener.accept().unwrap().0;

            let request: Request = parse_request_header(
                read_connection(&stream)
            );

            let res: Response = handle_connection(request, &self.routes);

            let response_header: String = format!(
                "HTTP/1.1 {}\r\n{}\r\n\r\n{}", 
                res.get_status(), 
                res.get_header(), 
                res.get_body()
            );

            stream.write_all(response_header.as_bytes())
            .expect("Unable to write response");
            
            stream.flush()
            .expect("Unable to flush output stream");
        }
    }
}

fn read_connection(mut stream: &TcpStream) -> String{
    const BUFFER_SIZE: usize = 4096;
    let mut buffer: [u8; 4096] = [0; BUFFER_SIZE];

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

fn handle_connection(
    request:    Request, 
    routes:     &Vec<Route>
) -> Response{

    let mut res: Response = Response{
        status: 200, 
        body: String::from(""), 
        headers: String::from("")
    };

    for route in routes{
        if request.path != route.path{
            continue;
        }
        else if request.method != route.method{
            continue;
        }
        res = route.get_func()(res);
    }

    return res;
}
