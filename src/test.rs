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
// left: `"#R*E"`,
// right: `"#*E"`', src/test.rs:79:26

    let stroke = Key::ControlNum | Key::LeftR | Key::MiddleStar | Key::MiddleE;
    assert_eq!(&stroke.to_string(), "#R*E");

    Ok(())
}
