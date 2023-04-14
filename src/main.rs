mod libs;

use libs::router::Route;
use libs::server::Server;

fn test(){
    println!("test");
}

fn test2(){
    println!("test2");
}

fn main(){
    let route = Route{func: test, path: String::from("/"), method: String::from("GET")};
    let route2 = Route{func: test2, path: String::from("/"), method: String::from("POST")};

    let routes = vec![route, route2];

    let server = Server{host: String::from("127.0.0.1"), port: 5000, routes: routes};
    server.erupt();
}
