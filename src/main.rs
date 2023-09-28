use response::res::Response;
use server::route::Route;
use server::httpserver::Server;

mod server;
mod request;
mod response;
mod func;

fn test(mut r: Response) -> Response{
    
    return r;
}

fn test2(mut r: Response) -> Response{

    r.set_status(404);
    
    return r;
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