use std::collections::HashMap;

use lava_http::response::Response;
use lava_http::request::Request;

use lava_http::server::Route;
use lava_http::server::Server;


fn test(mut res: Response, req: &Request) -> Response{

    res.redirect("/test");

    return res;
}

fn test2(mut res: Response, req: &Request) -> Response{

    res.send_json("{\"test\": \"dogs\"}");

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