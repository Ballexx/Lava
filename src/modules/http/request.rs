use std::{collections::HashMap};

pub struct Request{
    pub method:   String,
    pub route:    String,
    pub headers:  HashMap<String, String>,
    pub body:     String
}

pub fn read_request(request: String) -> Request{
    let header_body_split: Vec<&str> = request.split("\r\n\r\n").collect();
    let body = String::from("");

    if header_body_split.len() == 0{
        panic!("Reading empty request");
    }

    let headers: Vec<&str> = header_body_split[0].split("\r\n").collect();
    let pre_headers: Vec<&str> = headers[0].split(" ").collect();

    let method = String::from(pre_headers[0]);
    let route = String::from(pre_headers[1]);

    let mut dict_headers: HashMap<String, String> = HashMap::new();

    for i in 1..headers.len(){
        let (key, val) = headers[i].split_once(": ").unwrap();
        dict_headers.insert(String::from(key), String::from(val));
    }

    let parsed_request = Request{
        method: method,
        route: route,
        headers: dict_headers,
        body: body
    };

    return parsed_request;
}