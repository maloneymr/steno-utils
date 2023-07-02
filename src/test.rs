use super::*;

use lazy_static::lazy_static;
use serde_json::Value;

lazy_static! {
    static ref MAIN: Value = {
        let main_json_string = std::fs::read_to_string("main.json").unwrap();
        serde_json::from_str(&main_json_string).unwrap()
    };
}

#[test]
fn key_to_stroke() {
    let stroke: Stroke = Key::LeftS.into();
    assert!(stroke.contains(Key::LeftS));
    assert!(!stroke.contains(Key::RightS));
}

#[test]
fn or_key_to_stroke() {
    let stroke: Stroke = Key::LeftS.into();
    assert!(stroke.contains(Key::LeftS));
    assert!(!stroke.contains(Key::MiddleStar));
    assert!(!stroke.contains(Key::RightS));

    let stroke2: Stroke = stroke | Key::RightS;
    assert!(stroke2.contains(Key::LeftS));
    assert!(!stroke2.contains(Key::MiddleStar));
    assert!(stroke2.contains(Key::RightS));

    let stroke3: Stroke = Key::LeftP | Key::RightP;
    assert!(stroke3.contains(Key::LeftP));
    assert!(!stroke3.contains(Key::MiddleStar));
    assert!(stroke3.contains(Key::RightP));
}

#[test]
fn stroke_display() {
    let stroke: Stroke = Key::LeftS.into();
    assert_eq!(stroke.to_string(), "S");

    let stroke2: Stroke = stroke | Key::RightS;
    assert_eq!(stroke2.to_string(), "S-S");

    let stroke3: Stroke = Key::RightS.into();
    assert_eq!(stroke3.to_string(), "-S");

    let stroke4: Stroke = Key::LeftP | Key::RightS;
    assert_eq!(stroke4.to_string(), "P-S");

    let stroke5: Stroke = Key::LeftS | Key::MiddleStar | Key::RightP;
    assert_eq!(stroke5.to_string(), "S*P");
}

#[test]
fn parse_stroke() -> anyhow::Result<()> {
    let strokes = &[
        "A",
        "SA",
        "AS",
        "SAS",
        "S-S",
    ];
    for stroke in strokes {
        let s: Stroke = Stroke::parse(stroke)?;
        assert_eq!(&&s.to_string(), stroke);
    }

    Ok(())
}

#[test]
fn test_all_main_parses() {
    for (outline, _word) in MAIN.as_object().unwrap() {
        let strokes = outline.split("/");
        for stroke in strokes {
            match Stroke::parse(stroke) {
                Ok(s) => assert_eq!(&s.to_string(), stroke),
                Err(e) => panic!("Failed to parse {stroke:?}: {e:?}"),
            }
        }
    }
}

#[test]
fn regression() -> anyhow::Result<()> {
    let stroke = Key::ControlNum | Key::LeftR | Key::MiddleStar | Key::MiddleE;
    assert_eq!(&stroke.to_string(), "#R*E");

    Ok(())
}

#[test]
fn plover_dict_parse() {
    use plover_dict::Token;
    assert_eq!(Token::parse("Hello"), Token::Word("Hello".to_string()));
    assert_eq!(Token::parse("{^}"), Token::Attach);
    assert_eq!(Token::parse("{&a}"), Token::Glue("a".to_string()));
    assert_eq!(Token::parse("{^ab}"), Token::Suffix("ab".to_string()));
    assert_eq!(Token::parse("{cd^}"), Token::Prefix("cd".to_string()));
}

#[test]
fn test_dictionary() -> anyhow::Result<()> {
    use super::dictionary::Dictionary;
    use super::plover_dict::Token;

    let dictionary = Dictionary::new();

    let outline = Outline::parse("AFT")?;
    assert_eq!(dictionary.lookup(outline), Some(Token::Word("after".to_string())));

    let outline = Outline::parse("HE")?;
    assert_eq!(dictionary.lookup(outline), Some(Token::Word("he".to_string())));

    let outline = Outline::parse("WHA")?;
    assert_eq!(dictionary.lookup(outline), Some(Token::Word("what".to_string())));

    let outline = Outline::parse("TKE/SREL/OP")?;
    assert_eq!(dictionary.lookup(outline), Some(Token::Word("develop".to_string())));

    Ok(())
}
