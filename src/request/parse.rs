use std::collections::HashMap;

use crate::func::hashmapper::hashmapify_headers;
use super::req::Request;

pub fn parse_request_header(header: String) -> Request{
    let body_split: Vec<&str> = header.split("\r\n\r\n").collect();

    let request_header: String = String::from(body_split[0]);
    let mut request_body: String = String::from("");

    if body_split.len() > 1{
        request_body = String::from(body_split[1]);
    }

    let mut raw_headers: Vec<&str> = request_header.split("\r\n").collect();
    let raw_title_header: &str = raw_headers[0];

    raw_headers.remove(0);

    let raw_title_header_string: String = String::from(raw_title_header);
    let title_header: Vec<&str> = raw_title_header_string.split(" ").collect();

    let method: String = String::from(title_header[0]);
    let mut path: String = String::from("");

    if title_header.len() > 1{
        path = String::from(title_header[1]);
    }

    let hashmapped_headers: HashMap<String, String> = hashmapify_headers(raw_headers);

    return Request { 
        method: method, 
        path: path, 
        body: request_body, 
        headers: hashmapped_headers
    };
}