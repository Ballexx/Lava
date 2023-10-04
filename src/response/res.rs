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
    /// Set response-status
    /// 
    /// Example
    /// ```
    /// res.set_status(404);
    /// ```
    pub fn set_status(&mut self, status: i32){
        
        if valid::is_valid_status(status){
            self.status = status;
        }
        else{
            panic!("Invalid response status");
        }
    }

    /// Get response-status
    /// 
    /// Example
    /// ```
    /// res.get_status();
    /// ```
    pub fn get_status(&self) -> i32{
        return self.status;
    }

    /// Send response-body as text
    /// 
    /// Example
    /// ```
    /// res.send_body("body content");
    /// ```
    pub fn send_body(&mut self, body: &str){

        let string_body: String = String::from(body);

        if body.len() < MAX_CONTENT_LEN{
            self.body = string_body;
        }
        else{
            panic!("Response-body is too large! Max is {}", MAX_CONTENT_LEN);
        }
    }
    
    /// Send response-body as file
    /// 
    /// Example
    /// ```
    /// res.send_file("path.html");
    /// ```
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

    /// Get response-body
    /// 
    /// Example
    /// ```
    /// res.get_body();
    /// ```
    pub fn get_body(&self) -> String{
        return String::from(&self.body);
    }

    fn get_header_len(&self, header: &str) -> String{
        if self.headers.len() > 0 {

            return format!("\r\n{}", header);
        }
        else {

            return String::from(header);           
        }
    }


    /// Send response-headers in the form of a hashmap (this will overwrite an already set header for the response)
    /// 
    /// Example
    /// ```
    /// let mut headers: HashMap<&str, &str> = HashMap::new();
    /// 
    /// headers.insert("key", "value");
    /// 
    /// res.set_header(&headers);
    /// ```
    pub fn set_header(&mut self, headers: &HashMap<&str, &str>){

        let stringified_header: String = stringify_hashmapped_headers(headers);

        self.headers = stringified_header;
    }

    /// Append response-headers in the form of a hashmap (this will append to the already set header for the response)
    /// 
    /// Example
    /// ```
    /// let mut headers: HashMap<&str, &str> = HashMap::new();
    /// 
    /// headers.insert("key", "value");
    /// 
    /// res.append_header(&headers);
    /// ```
    pub fn append_header(&mut self, headers: &HashMap<&str, &str>){

        let stringified_header: String = stringify_hashmapped_headers(headers);

        self.headers.push_str(
            &self.get_header_len(&stringified_header)
        );
    }

    /// Clear set response-header
    /// 
    /// Example
    /// ```
    /// res.clear_header();
    /// ```
    pub fn clear_header(&mut self){
        self.headers = String::from("");
    }

    /// Set response-header in the form of a string (this will overwrite an already set header for the response)
    /// 
    /// Example
    /// ```
    /// res.set_string_header("Key: Value");
    /// ```
    pub fn set_string_header(&mut self, header: String){
        self.headers = header;
    }

    /// Append response-header in the form of a string (this will append to the already set header for the response)
    /// 
    /// Example
    /// ```
    /// res.append_string_header("Key: Value");
    /// ```
    pub fn append_string_header(&mut self, header: String){
        self.headers.push_str(
            &self.get_header_len(&header)
        );
    }

    /// Get response-header
    /// 
    /// Example
    /// ```
    /// res.get_header();
    /// ```
    pub fn get_header(&self) -> String{
        return String::from(&self.headers);
    }
    /// Redirects to given path
    /// 
    /// Example
    /// ```
    /// res.redirect("/path");
    /// ```
    pub fn redirect(&mut self, path: &str){

        self.headers.push_str(
            &self.get_header_len(&format!("Location: {}", path))
        );

        self.set_status(303);
    }


    pub fn send_json(&mut self, json: &str){
        self.headers.push_str(
            &self.get_header_len("Content-Type: application/json")
        );

        self.body = String::from(json);
    }


}