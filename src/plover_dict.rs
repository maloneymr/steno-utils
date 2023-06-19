// https://github.com/openstenoproject/plover/blob/master/plover/system/english_stenotype.py
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref GLUE_RE: Regex = Regex::new(r"^\{&([^}]+)\}").unwrap();
    static ref PREFIX_RE: Regex = Regex::new(r"^\{([^}]+)\^\}").unwrap();
    static ref SUFFIX_RE: Regex = Regex::new(r"^\{\^([^}]+)\}").unwrap();
    static ref INFIX_RE: Regex = Regex::new(r"^\{\^([^}]+)\^\}").unwrap();
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DictionaryEntry(Vec<Part>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Part {
    Text(String),
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

impl Part {
    pub fn parse(word: &str) -> Part {
        if word == "{^}" {
            return Part::Attach;
        }

        if let Some(captures) = GLUE_RE.captures(word) {
            return Part::Glue(captures.get(1).unwrap().as_str().to_string());
        }

        if let Some(captures) = INFIX_RE.captures(word) {
            return Part::Infix(captures.get(1).unwrap().as_str().to_string());
        }

        if let Some(captures) = PREFIX_RE.captures(word) {
            return Part::Prefix(captures.get(1).unwrap().as_str().to_string());
        }

        if let Some(captures) = SUFFIX_RE.captures(word) {
            return Part::Suffix(captures.get(1).unwrap().as_str().to_string());
        }

        Part::Text(word.to_string())
    }
}
