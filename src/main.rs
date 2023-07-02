use steno_utils;
use steno_utils::plover_dict::Token;

use lazy_static::lazy_static;
use serde_json::Value;
use serialport::SerialPort;


lazy_static! {
    static ref MAIN: Value = {
        let main_json_string = std::fs::read_to_string("main.json").unwrap();
        serde_json::from_str(&main_json_string).unwrap()
    };
}

fn main() {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in &ports {
        println!("{}", p.port_name);
    }

    /*
    println!("{}", &ports[0].port_name);
    let port_filename = &ports[0].port_name; //"/dev/ttyACM0";
    for (_outline, word) in MAIN.as_object().unwrap() {
        let part = Token::parse(word.as_str().unwrap());
        println!("{:<30}{part:?}", word.as_str().unwrap());
    }
    */
}
