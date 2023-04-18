use core::panic;
use std::{fs, collections::HashMap};

pub struct Response{
    pub body:   String,
    pub header: String,
    pub status: i32
}

pub fn stringify_file(path: String) -> String{
    let file_content = fs::read_to_string(path);

    match file_content {
        Ok(_) => {
            return file_content.unwrap();
        }
        Err(_) => {
            panic!("Error reading file");       
        }
    }
}

pub fn build_header(headers: HashMap<&str, &str>) -> String{
    let mut header: String = String::from("");

    for(key, value) in headers{
        header = format!("{}\r\n{}: {}", header, key, value);
    }

    return header;
}

impl Response{
    pub fn send(&self){
        
    }
}
