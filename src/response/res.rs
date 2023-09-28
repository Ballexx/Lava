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
}