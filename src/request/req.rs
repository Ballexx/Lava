use std::collections::HashMap;

pub struct Request{
    pub method:     String,
    pub path:       String,
    pub body:       String,
    pub headers:    HashMap<String, String>
}

impl Request {
    pub fn get_method(&self) -> &String{
        return &self.method;
    }

    pub fn get_path(&self) -> &String{
        return &self.path;
    }

    pub fn get_body(&self) -> &String{
        return &self.body;
    }
}

pub trait handle_header{
    fn get_header(&self) -> &Self;
    fn get_key(&self, key: &str) -> String;
    fn key_exist(&self, key: &str) -> bool;
}

impl handle_header for Request{
    fn get_header(&self) -> &Self { self }
    
    fn get_key(&self, key: &str) -> String{
        match self.headers.get(key){
            Some(key) => {
                return String::from(key);
            }
            None => { panic!("Key does not exist") }
        }
    }
    fn key_exist(&self, key: &str) -> bool {
        match self.headers.get(key){
            Some(_) => { return true }
            None => { return false }
        }
    }
    
}


