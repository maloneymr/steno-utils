// https://github.com/openstenoproject/plover/blob/master/plover/system/english_stenotype.py
use lazy_static::lazy_static;
use serde_json::Value;
use regex::Regex;

lazy_static! {
    static ref GLUE_RE: Regex = Regex::new(r"^\{&([^}]+)\}").unwrap();
    static ref PREFIX_RE: Regex = Regex::new(r"^\{([^}]+)\^\}").unwrap();
    static ref SUFFIX_RE: Regex = Regex::new(r"^\{\^([^}]+)\}").unwrap();
    static ref INFIX_RE: Regex = Regex::new(r"^\{\^([^}]+)\^\}").unwrap();
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DictionaryEntry(Vec<Token>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    Word(String),
    Prefix(String),
    Suffix(String),
    Infix(String),
    Glue(String),
    Attach,
    CapNext,
    CapPrev,
    UncapNext,
    UncapPrev,
}

impl Token {
    pub fn parse(word: &str) -> Token {
        if word == "{^}" {
            return Token::Attach;
        }

        if let Some(captures) = GLUE_RE.captures(word) {
            return Token::Glue(captures.get(1).unwrap().as_str().to_string());
        }

        if let Some(captures) = INFIX_RE.captures(word) {
            return Token::Infix(captures.get(1).unwrap().as_str().to_string());
        }

        if let Some(captures) = PREFIX_RE.captures(word) {
            return Token::Prefix(captures.get(1).unwrap().as_str().to_string());
        }

        if let Some(captures) = SUFFIX_RE.captures(word) {
            return Token::Suffix(captures.get(1).unwrap().as_str().to_string());
        }

        Token::Word(word.to_string())
    }
}


lazy_static! {
    static ref MAIN: Value = {
        let main_json_string = std::fs::read_to_string("main.json").unwrap();
        serde_json::from_str(&main_json_string).unwrap()
    };
}

pub struct Machine {
    history: Vec<Token>,
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            history: vec![],
        }
    }

    pub fn input(&mut self, token: Token) {
        self.history.push(token);
    }

    pub fn commit(&mut self) -> String {
        self.history = vec![];
        "".to_string()
    }
}

