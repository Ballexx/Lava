pub struct Route{
    pub func:      fn(),
    pub path:      String,
    pub method:    String
}

impl Route{
    pub fn get_func(&self) -> fn(){
        return self.func;
    }

    pub fn get_path(&self) -> &String{
        return &self.path;
    }

    pub fn get_method(&self) -> &String{
        return  &self.method;
    }
}