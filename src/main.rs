mod modules;

use modules::router::Route;
use modules::server::Server;

fn test(){
    println!("test");
}

fn test2(){
    println!("test2");
}

fn main(){
    let route = Route{func: test, path: String::from("/"), method: String::from("GET")};
    let route2 = Route{func: test2, path: String::from("/"), method: String::from("POST")};

    let mut routes = vec![route, route2];

    let server = Server{host: String::from("127.0.0.1"), port: 5000, mount: routes};
    server.listen();
}
