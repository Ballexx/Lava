mod server;
mod request;
mod response;
mod func;

use response::res::Response;
use request::req::Request;
use server::route::Route;
use server::httpserver::Server;


fn test(mut res: Response, req: &Request) -> Response{
    
    return res;
}

fn test2(mut res: Response, req: &Request) -> Response{

    res.set_status(404);
    
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