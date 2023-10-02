use crate::{response::valid, func::hashmapper::stringify_hashmapped_headers};
use core::panic;
use std::{fs, collections::HashMap};

pub struct Response{
    pub status:     i32,
    pub body:       String,
    pub headers:    String
}

const MAX_CONTENT_LEN: usize = 1000000000;

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

        if body.len() < MAX_CONTENT_LEN{
            self.body = body;
        }
        else{
            panic!("Response-body is too large! Max is {}", MAX_CONTENT_LEN);
        }
    }
    
    pub fn send_file(&mut self, path: &str){

        let file_content = fs::read_to_string(path);

        match file_content {
            Ok(_) => {

                let content: String = file_content.unwrap();
                
                if content.len() > MAX_CONTENT_LEN{
                    panic!("Response-body is too large! Max is {}", MAX_CONTENT_LEN);
                }

                self.body = content;
            }
            Err(err) => {
                panic!("{}", err);     
            }
        }
    }

    fn get_header_len(&self, content: &str) -> String{
        if self.headers.len() > 0 {

            return format!("\r\nLocation: {}", content);
        }
        else {

            return format!("Location: {}", content);            
        }
    }

    pub fn get_body(&self) -> String{
        return String::from(&self.body);
    }

    pub fn set_header(&mut self, headers: &HashMap<&str, &str>){

        let stringified_header: String = stringify_hashmapped_headers(headers);

        self.headers = stringified_header;
    }

    pub fn append_header(&mut self, headers: &HashMap<&str, &str>){

        let stringified_header: String = stringify_hashmapped_headers(headers);

        self.headers.push_str(
            &self.get_header_len(&stringified_header)
        );
    }

    pub fn clear_header(&mut self){
        self.headers = String::from("");
    }

    pub fn set_string_header(&mut self, header: String){
        self.headers = header;
    }
    
    pub fn append_string_header(&mut self, header: String){
        self.headers.push_str(&header);
    }

    pub fn get_header(&self) -> String{
        return String::from(&self.headers);
    }

    pub fn redirect(&mut self, path: &str){

        self.headers.push_str(
            &self.get_header_len(path)
        );

        self.set_status(303);
    }
}