use std::collections::HashMap;

use lava::response::Response;
use lava::request::Request;

use lava::server::Route;
use lava::server::Server;


fn functions(mut res: Response, req: &Request) -> Response{

    req.get_body();

    req.get_method();

    req.get_path();

    req.get_header().get_key("key");

    req.get_header().key_exist("key");

    let mut headers: HashMap<&str, &str> = HashMap::new();
    headers.insert("Test", "Dogs");
    headers.insert("dows", "dsdsa");
    res.set_header(&headers);
    
    res.clear_header();

    res.set_status(404);

    res.send_file("test.html");

    res.send_body(String::from("<h1 style='color: red;'>I hate dog</h1><ul><li>kuk</li><li>dsadsa</li></ul>"));

    return res;
}

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

    res.set_header(&headers);

    let mut headers2: HashMap<&str, &str> = HashMap::new();
    headers2.insert("Tesdsadsat", "Dodsags");
    headers2.insert("dossws", "dsdssssa");

    res.append_header(&headers2);

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