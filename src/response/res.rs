use crate::response::valid_res;

pub struct Response{
    pub status: i32,
    pub body: String,
    pub headers: String
}

impl Response{
    pub fn set_status(&mut self, val: i32){
        
        if valid_res::is_valid_status(val){
            self.status = val;
        }
        else{
            panic!("Invalid response status");
        }
    }
    pub fn get_status(&self) -> i32{
        return self.status;
    }

    pub fn set_body(&mut self, body: String){
        let max_content_len: usize = 1000000000;

        if body.len() < max_content_len{
            self.body = body;
        }
        else{
            panic!("Response-body is too large! Max is {}", max_content_len);
        }
    }
    pub fn get_body(&self) -> String{
        return String::from(&self.body);
    }

    pub fn set_header(&mut self, header: String){
        self.headers = header;
    }
    pub fn get_header(&self) -> String{
        return String::from(&self.headers);
    }
}