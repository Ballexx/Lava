use std::collections::HashMap;

pub fn hashmapify_headers(headers: Vec<&str>) -> HashMap<String, String>{
    
    let mut hashmapped_headers: HashMap<String, String> = HashMap::new();

    for i in 1..headers.len(){
        let (key, val) = headers[i].split_once(": ").unwrap();
        hashmapped_headers.insert(String::from(key), String::from(val));
    }

    return hashmapped_headers;
}

pub fn stringify_hashmapped_headers(headers: HashMap<&str, &str>) -> String{

    let mut string_header: String = String::from("");

    for (key, value) in headers{
        
        let formated = format!(
            "{}: {}\r\n", 
            String::from(key), 
            String::from(value)
        );

        string_header.push_str(&formated);
    }

    string_header.truncate(string_header.len() - 4);

    return string_header;
}