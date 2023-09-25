mod server;
mod request;
mod response;
mod func;

fn test(){
    
}

fn test2(){
    println!("tesdsadsaddt");
}

fn main(){
    let route = server::route::Route{func: test, path: String::from("/"), method: String::from("GET")};
    let route2 = server::route::Route{func: test2, path: String::from("/test"), method: String::from("GET")};

    let routes = vec![route, route2];

    let server = server::httpserver::Server{host: String::from("127.0.0.1"), port: 5000, routes: routes};
    server.erupt();
}