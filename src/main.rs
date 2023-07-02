use steno_utils;
use steno_utils::plover_dict::Token;
use steno_utils::{Stroke, Key};
use steno_utils::Dictionary;
use std::time::Duration;

use lazy_static::lazy_static;
use serde_json::Value;
use serialport::{SerialPort, SerialPortInfo};


lazy_static! {
    static ref MAIN: Value = {
        let main_json_string = std::fs::read_to_string("main.json").unwrap();
        serde_json::from_str(&main_json_string).unwrap()
    };
}

fn main() {
    let port_filename = guess_port();
    let mut port = serialport::new(port_filename, 9600)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open port");

    let dictionary = Dictionary::new();

    loop {
        if let Some(stroke) = read_stroke(port.as_mut()) {
            let token = dictionary.lookup(stroke.into());
            println!("{stroke} => {token:?}");
        }
    }
}

fn guess_port() -> String {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in &ports {
        println!("{}", &p.port_name);
    }
    ports[0].port_name.clone()
}

fn read_stroke(port: &mut dyn SerialPort) -> Option<Stroke> {
    let mut buf = [0; 6];
    let mut total_amount = 0;

    loop {
        let buf_slice = &mut buf[total_amount..6];
        match port.read(buf_slice) {
            Ok(amount) => {
                total_amount += amount;
            },
            Err(_e) => {
            }
        }

        if total_amount == 6 {
            break;
        }
    }

    let value: u64 =
        (buf[0] as u64) |
        (buf[1] as u64) << 8 |
        (buf[2] as u64) << 16 |
        (buf[3] as u64) << 24 |
        (buf[4] as u64) << 32 |
        (buf[5] as u64) << 40;

    let mut keys = vec![];
    for (key, key_value) in KEY_CODES {
        if value & key_value == *key_value {
            keys.push(*key);
        }
    }

    if keys.len() > 0 {
        Some(Stroke::from_keys(keys.as_slice()))
    } else {
        None
    }
}

const KEY_CODES: &[(Key, u64)] = &[
    (Key::LeftS, 0x000000002080),
    (Key::LeftT, 0x000000001080),
    (Key::LeftK, 0x000000000880),
    (Key::LeftP, 0x000000000480),
    (Key::LeftW, 0x000000000280),
    (Key::LeftH, 0x000000000180),
    (Key::LeftR, 0x000000400080),

    (Key::MiddleA, 0x000000200080),
    (Key::MiddleO, 0x000000100080),
    (Key::MiddleStar, 0x000000080080),
    (Key::MiddleStar, 0x000020000080),
    (Key::MiddleStar, 0x000000040080),
    (Key::MiddleStar, 0x000010000080),
    (Key::MiddleE, 0x000008000080),
    (Key::MiddleU, 0x000004000080),

    (Key::RightF, 0x000002000080),
    (Key::RightR, 0x000001000080),
    (Key::RightP, 0x004000000080),
    (Key::RightB, 0x002000000080),
    (Key::RightL, 0x001000000080),
    (Key::RightG, 0x000800000080),
    (Key::RightT, 0x000400000080),
    (Key::RightS, 0x000200000080),
    (Key::RightD, 0x000100000080),
    (Key::RightZ, 0x010000000080),
];

const KEY_CHARS: &[(Key, char)] = &[
    (Key::LeftS, 'S'),
    (Key::LeftT, 'T'),
    (Key::LeftK, 'K'),
    (Key::LeftP, 'P'),
    (Key::LeftW, 'W'),
    (Key::LeftH, 'H'),
    (Key::LeftR, 'R'),

    (Key::MiddleA, 'A'),
    (Key::MiddleO, 'O'),
    (Key::MiddleStar, '*'),
    (Key::MiddleE, 'E'),
    (Key::MiddleU, 'U'),

    (Key::RightF, 'F'),
    (Key::RightR, 'R'),
    (Key::RightP, 'P'),
    (Key::RightB, 'B'),
    (Key::RightL, 'L'),
    (Key::RightG, 'G'),
    (Key::RightT, 'T'),
    (Key::RightS, 'S'),
    (Key::RightD, 'D'),
    (Key::RightZ, 'Z'),
];
