use std::collections::HashMap;

pub struct Request{
    pub method:     String,
    pub path:       String,
    pub body:       String,
    pub headers:    HashMap<String, String>
}