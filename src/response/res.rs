pub struct Response{
    pub status: i32,
    pub body: String,
    pub headers: String
}

impl Response{
    pub fn set_status(&mut self, val: i32){
        self.status = val;
    }
    pub fn get_status(&self) -> i32{
        return self.status;
    }

    pub fn set_body(&mut self, body: String){
        self.body = body;
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