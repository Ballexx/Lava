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

    pub fn get_headers(&self) -> &Self{
        self
    }
    pub fn get_key(&self, key: &str) -> &String{
        match &self.headers.get(key) {
            Some(&key) => { return &key; },
            _ => {
                println!("Key does not exist");
                return &String::from("");
            }
        }
    }
}


