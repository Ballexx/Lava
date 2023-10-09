use std::collections::HashMap;

pub fn jsonify(json: &str) -> String{
    let replaced_json: String = String::from(
        json
    ).replace("'", "\"");

    return replaced_json;
}

pub fn de_jsonify(json: &str) -> HashMap<String, V>{
    let data = String::from(json);

    data.replace("\r\n", "");

    return data;

}