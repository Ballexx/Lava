use std::collections::HashMap;

pub fn hashmapify_headers(headers: Vec<&str>) -> HashMap<String, String>{
    
    let mut hashmapped_headers: HashMap<String, String> = HashMap::new();

    for i in 1..headers.len(){
        let (key, val) = headers[i].split_once(": ").unwrap();
        hashmapped_headers.insert(String::from(key), String::from(val));
    }

    return hashmapped_headers;
}