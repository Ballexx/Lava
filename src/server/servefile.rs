use crate::response::res::Response;

pub fn send_static_file(mut path: String) -> Response{

    let mut res: Response = Response { 
        status: (200), 
        body: (String::from("")), 
        headers: (String::from("")) 
    };

    if path.chars().nth(0).unwrap() == '/'{

        path = path.split_off(1);
    }

    if path.ends_with(".css"){

        res.set_string_header(
            String::from("Content-Type: text/css")
        )
    }
    else if path.ends_with(".js"){
        
        res.set_string_header(
            String::from("Content-Type: text/javascript")
        )
    }

    res.send_file(&path);

    return res;
}