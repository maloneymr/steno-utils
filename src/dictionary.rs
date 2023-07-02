use lazy_static::lazy_static;
use serde_json::Value;

use super::plover_dict::Token;
use super::{Outline, Stroke};


lazy_static! {
    static ref MAIN: Value = {
        let main_json_string = std::fs::read_to_string("main.json").unwrap();
        serde_json::from_str(&main_json_string).unwrap()
    };
}

pub struct Dictionary {
    _private: std::marker::PhantomData<u8>,
}

impl Dictionary {
    pub fn new() -> Dictionary {
        Dictionary {
            _private: std::marker::PhantomData::default(),
        }
    }

    pub fn lookup(&self, outline: Outline) -> Option<Token> {
        let outline_str = outline.to_string();
        if let Some(val) = MAIN.get(&outline_str) {

            let token = Token::parse(&val.as_str()?);
            return Some(token);
        }
        None
    }
}
