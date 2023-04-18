mod libs;

use libs::http::response;
use libs::router::Route;
use libs::server::Server;
use std::collections::HashMap;

fn test(){
    println!("test");
}

fn test2(){

    let headers = HashMap::from([
        ("test", "cool"),
        ("lol", "ok"),
    ]);
    let string_headers = response::build_header(headers);
    
    let response = response::Response{status: 200, body:"".to_string(), header:"".to_string()};
}

fn main(){
    let route = Route{func: test, path: String::from("/"), method: String::from("GET")};
    let route2 = Route{func: test2, path: String::from("/test"), method: String::from("GET")};

    let routes = vec![route, route2];

    let server = Server{host: String::from("127.0.0.1"), port: 5000, routes: routes};
    server.erupt();
}