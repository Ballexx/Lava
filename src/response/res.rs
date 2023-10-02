use crate::{response::valid, func::hashmapper::stringify_hashmapped_headers};
use core::panic;
use std::{fs, collections::HashMap};

pub struct Response{
    pub status:     i32,
    pub body:       String,
    pub headers:    String
}

impl Response{
    pub fn set_status(&mut self, status: i32){
        
        if valid::is_valid_status(status){
            self.status = status;
        }
        else{
            panic!("Invalid response status");
        }
    }
    pub fn get_status(&self) -> i32{
        return self.status;
    }

    pub fn send_body(&mut self, body: String){

        let max_content_len: usize = 1000000000;

        if body.len() < max_content_len{
            self.body = body;
        }
        else{
            panic!("Response-body is too large! Max is {}", max_content_len);
        }
    }
    
    pub fn send_file(&mut self, path: &str){

        let max_content_len: usize = 1000000000;

        let file_content = fs::read_to_string(path);

        match file_content {
            Ok(_) => {

                let content = file_content.unwrap();
                
                if content.len() > max_content_len{
                    panic!("Response-body is too large! Max is {}", max_content_len);
                }

                self.body = content;
            }
            Err(err) => {
                panic!("{}", err);     
            }
        }
    }

    pub fn get_body(&self) -> String{
        return String::from(&self.body);
    }

    pub fn set_header(&mut self, headers: HashMap<&str, &str>){

        let stringified_header: String = stringify_hashmapped_headers(headers);

        self.body = stringified_header;
    }
    
    pub fn append_header(&mut self, headers: HashMap<&str, &str>){
        
        let stringified_header: String = stringify_hashmapped_headers(headers);

        self.body.push_str(&stringified_header);
    }

    pub fn set_string_header(&mut self, header: String){
        self.headers = header;
    }

    pub fn get_header(&self) -> String{
        return String::from(&self.headers);
    }

    pub fn redirect(&self){

    }
}