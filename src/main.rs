mod server;
mod request;
mod response;
mod func;

use std::collections::HashMap;

use response::res::Response;
use request::req::{Request, handle_header};
use server::route::Route;
use server::server::Server;


fn test(mut res: Response, req: &Request) -> Response{

    res.send_file("test.html");
    return res;
}

fn test2(mut res: Response, req: &Request) -> Response{

    res.set_status(404);
    res.send_body(String::from("<h1 style='color: red;'>I hate dog</h1><ul><li>kuk</li><li>dsadsa</li></ul>"));
    
    let mut headers: HashMap<&str, &str> = HashMap::new();
    headers.insert("Test", "Dogs");
    headers.insert("dows", "dsdsa");

    res.set_header(headers);

    let key = req.get_header().key_exist("dsdsad");
    println!("{}", key);

    return res;
}

fn main(){
    let route: Route = Route{
        handler: test, 
        path: "/", 
        method: "GET"
    };
    
    let route2: Route = Route{
        handler: test2, 
        path: "/test", 
        method: "GET"
    };

    let routes: Vec<Route> = vec![route, route2];

    let server: Server = Server{
        host: "127.0.0.1", 
        port: 5000, 
        routes: routes
    };

    server.erupt();
}