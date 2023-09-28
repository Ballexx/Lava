use crate::response::res::Response;

pub struct Route{
    pub handler:      fn(Response) -> Response,
    pub path:      &'static str,
    pub method:    &'static str
}

impl Route{
    pub fn get_func(&self) 
    -> fn(Response) 
    -> Response{
        return self.handler;
    }

    pub fn get_path(&self) -> String{
        let path: String = String::from(self.path);
        
        return path;
    }

    pub fn get_method(&self) -> String{
        let method: String = String::from(self.method);

        return method;
    }
}