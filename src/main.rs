use steno_utils;
use steno_utils::plover_dict::Part;

use lazy_static::lazy_static;
use serde_json::Value;

lazy_static! {
    static ref MAIN: Value = {
        let main_json_string = std::fs::read_to_string("main.json").unwrap();
        serde_json::from_str(&main_json_string).unwrap()
    };
}

fn main() {
    for (_outline, word) in MAIN.as_object().unwrap() {
        let part = Part::parse(word.as_str().unwrap());
        println!("{:<30}{part:?}", word.as_str().unwrap());
    }
}
