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
    request::parse::parse_request_header,
    request::req::Request,
    server::route::Route,
    response::res::Response
};

use crate::server::servefile::send_static_file;

pub struct Server{
    pub host:      &'static str,
    pub port:      i32,
    pub routes:    Vec<Route>
}

impl Server{
    fn bind(&self) -> TcpListener{

        let (
            port_min, 
            port_max
        ) = (1023, 65535);

        if self.port < port_min || self.port > port_max{
            panic!("Invalid port");
        }

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

    /// Listens to server on given host and port
    /// 
    /// Example
    /// ```
    /// let server: Server = Server{
    ///     host: "127.0.0.1",
    ///     port: 5000,
    ///     routes: routes
    /// };
    /// ```
    pub fn erupt(&self){
        let listener: TcpListener = self.bind();
        
        loop {
            let mut stream: TcpStream;

            match listener.accept(){
                Ok(conn) => {
                    stream = conn.0;
                }
                Err(err) => { panic!("{}", err) }
            };

            let request: Request = parse_request_header(
                read_connection(&stream)
            );

            let mut res: Response = Response { 
                status: (200), 
                body: (String::from("")), 
                headers: (String::from("")) 
            };

            if request.path.ends_with(".css") || request.path.ends_with(".js"){
                res = send_static_file(request.path);
            }
            else{ 
                res = handle_connection(request, &self.routes);
            }

            let response_header: String = format!(
                "HTTP/1.1 {}\r\n{}\r\n\r\n{}", 
                res.get_status(), 
                res.get_header(), 
                res.get_body()
            );

            match stream.write_all(response_header.as_bytes()){
                Ok(_) => {}
                Err(err) => { panic!("{}", err) }
            }

            match stream.flush(){
                Ok(_) => {}
                Err(err) => { panic!("{}", err) }
            }
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
    req:        Request, 
    routes:     &Vec<Route>
) -> Response{

    let mut res: Response = Response{
        status: 200, 
        body: String::from(""), 
        headers: String::from("")
    };

    for route in routes{
        if req.path != route.path{
            continue;
        }
        else if req.method != route.method{
            continue;
        }
        res = route.get_func()(res, &req);
    }

    return res;
}


