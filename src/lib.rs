use anyhow::anyhow;

#[cfg(test)]
mod test;

pub mod plover_dict;
pub mod dictionary;

#[derive(Clone, Eq, PartialEq)]
pub struct Outline(Vec<Stroke>);

impl std::fmt::Debug for Outline {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (i, stroke) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, "/")?;
            }
            write!(f, "{stroke}")?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Outline {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (i, stroke) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, "/")?;
            }
            write!(f, "{stroke}")?;
        }
        Ok(())
    }
}

impl std::ops::Div<Stroke> for Stroke {
    type Output = Outline;

    fn div(self, stroke: Stroke) -> Outline {
        Outline(vec![self, stroke])
    }
}

impl std::ops::Div<Stroke> for Outline {
    type Output = Outline;

    fn div(self, stroke: Stroke) -> Outline {
        let Outline(mut strokes) = self;
        strokes.push(stroke);
        Outline(strokes)
    }
}

impl Outline {
    pub fn parse(outline: &str) -> anyhow::Result<Outline> {
        let mut strokes = vec![];
        for stroke_str in outline.split('/') {
            let stroke = Stroke::parse(stroke_str)?;
            strokes.push(stroke);
        }

        Ok(Outline(strokes))
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Stroke(u32);

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u32)]
pub enum Key {
    ControlNum = 1 << 1,
    ControlLeft = 1 << 2,
    ControlRight = 1 << 3,

    LeftS = 1 << 4,
    LeftT = 1 << 5,
    LeftK = 1 << 6,
    LeftP = 1 << 7,
    LeftW = 1 << 8,
    LeftH = 1 << 9,
    LeftR = 1 << 10,

    MiddleA = 1 << 11,
    MiddleO = 1 << 12,
    MiddleStar = 1 << 13,
    MiddleE = 1 << 14,
    MiddleU = 1 << 15,

    RightF = 1 << 16,
    RightR = 1 << 17,
    RightP = 1 << 18,
    RightB = 1 << 19,
    RightL = 1 << 20,
    RightG = 1 << 21,
    RightT = 1 << 22,
    RightS = 1 << 23,
    RightD = 1 << 24,
    RightZ = 1 << 25,
}

impl From<Key> for Stroke {
    fn from(key: Key) -> Stroke {
        Stroke(key as u32)
    }
}

impl Stroke {
    pub fn contains(&self, key: Key) -> bool {
        self.0 & key as u32 != 0
    }

    fn is_valid(&self) -> bool {
        self.0 != 0
    }

    pub fn parse(mut stroke: &str) -> anyhow::Result<Stroke> {
        let mut result = Stroke(0);

        loop {
            let (opt_key, rest) = Stroke::consume_control(stroke)?;
            stroke = rest;
            if let Some(key) = opt_key {
                result |= key;
            } else {
                break;
            }
        }

        loop {
            let (opt_key, rest) = Stroke::consume_initial(stroke)?;
            stroke = rest;
            if let Some(key) = opt_key {
                result |= key;
            } else {
                break;
            }
        }

        loop {
            let (opt_key, rest) = Stroke::consume_middle(stroke)?;
            stroke = rest;
            if let Some(key) = opt_key {
                result |= key;
            } else {
                break;
            }
        }

        loop {
            let (opt_key, rest) = Stroke::consume_final(stroke)?;
            stroke = rest;
            if let Some(key) = opt_key {
                result |= key;
            } else {
                break;
            }
        }

        if !result.is_valid() {
            return Err(anyhow!("empty"));
        }
        Ok(result)
    }

    fn consume_control(stroke: &str) -> anyhow::Result<(Option<Key>, &str)> {
        let Some(ch) = stroke.chars().next() else {
            return Ok((None, stroke));
        };
        let key = match ch {
            '#' => Key::ControlNum,
            '^' => Key::ControlLeft,
            '+' => Key::ControlRight,
            _ => return Ok((None, stroke)),
        };
        let rest = &stroke[ch.len_utf8()..];

        Ok((Some(key), rest))
    }

    fn consume_initial(stroke: &str) -> anyhow::Result<(Option<Key>, &str)> {
        let Some(ch) = stroke.chars().next() else {
            return Ok((None, stroke));
        };
        let key = match ch {
            'S' => Key::LeftS,
            'T' => Key::LeftT,
            'K' => Key::LeftK,
            'P' => Key::LeftP,
            'W' => Key::LeftW,
            'H' => Key::LeftH,
            'R' => Key::LeftR,
            _ => return Ok((None, stroke)),
        };
        let rest = &stroke[ch.len_utf8()..];

        Ok((Some(key), rest))
    }

    fn consume_middle(stroke: &str) -> anyhow::Result<(Option<Key>, &str)> {
        let Some(ch) = stroke.chars().next() else {
            return Ok((None, stroke));
        };
        let key = match ch {
            'A' => Key::MiddleA,
            'O' => Key::MiddleO,
            '*' => Key::MiddleStar,
            'E' => Key::MiddleE,
            'U' => Key::MiddleU,
            '-' => {
                let rest = &stroke[ch.len_utf8()..];
                return Ok((None, rest));
            },
            _ => return Ok((None, stroke)),
        };
        let rest = &stroke[ch.len_utf8()..];

        Ok((Some(key), rest))
    }

    fn consume_final(stroke: &str) -> anyhow::Result<(Option<Key>, &str)> {
        let Some(ch) = stroke.chars().next() else {
            return Ok((None, stroke));
        };
        let key = match ch {
            'F' => Key::RightF,
            'R' => Key::RightR,
            'P' => Key::RightP,
            'B' => Key::RightB,
            'L' => Key::RightL,
            'G' => Key::RightG,
            'T' => Key::RightT,
            'S' => Key::RightS,
            'D' => Key::RightD,
            'Z' => Key::RightZ,
            _ => return Ok((None, stroke)),
        };
        let rest = &stroke[ch.len_utf8()..];

        Ok((Some(key), rest))
    }
}

impl std::ops::BitOr<Key> for Stroke {
    type Output = Stroke;

    fn bitor(self, key: Key) -> Stroke {
        Stroke(self.0 | key as u32)
    }
}

impl std::ops::BitOrAssign<Key> for Stroke {
    fn bitor_assign(&mut self, key: Key) {
        *self = Stroke(self.0 | key as u32)
    }
}

impl std::ops::BitOr<Key> for Key {
    type Output = Stroke;

    fn bitor(self, key: Key) -> Stroke {
        Stroke(self as u32 | key as u32)
    }
}

impl std::fmt::Display for Stroke {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut has_middle = false;
        if self.contains(Key::ControlNum) {
            write!(f, "#")?;
        }
        if self.contains(Key::ControlLeft) {
            write!(f, "^")?;
        }
        if self.contains(Key::ControlRight) {
            write!(f, "+")?;
        }

        // LEFTS
        if self.contains(Key::LeftS) {
            write!(f, "S")?;
        }
        if self.contains(Key::LeftT) {
            write!(f, "T")?;
        }
        if self.contains(Key::LeftK) {
            write!(f, "K")?;
        }
        if self.contains(Key::LeftP) {
            write!(f, "P")?;
        }
        if self.contains(Key::LeftW) {
            write!(f, "W")?;
        }
        if self.contains(Key::LeftH) {
            write!(f, "H")?;
        }
        if self.contains(Key::LeftR) {
            write!(f, "R")?;
        }

        // MIDDLES
        if self.contains(Key::MiddleA) {
            write!(f, "A")?;
            has_middle = true;
        }
        if self.contains(Key::MiddleO) {
            write!(f, "O")?;
            has_middle = true;
        }
        if self.contains(Key::MiddleStar) {
            write!(f, "*")?;
            has_middle = true;
        }
        if self.contains(Key::MiddleE) {
            write!(f, "E")?;
            has_middle = true;
        }
        if self.contains(Key::MiddleU) {
            write!(f, "U")?;
            has_middle = true;
        }

        if self.contains(Key::RightF) {
            if !has_middle {
                write!(f, "-")?;
                has_middle = true;
            }
            write!(f, "F")?;
        }
        if self.contains(Key::RightR) {
            if !has_middle {
                write!(f, "-")?;
                has_middle = true;
            }
            write!(f, "R")?;
        }
        if self.contains(Key::RightP) {
            if !has_middle {
                write!(f, "-")?;
                has_middle = true;
            }
            write!(f, "P")?;
        }
        if self.contains(Key::RightB) {
            if !has_middle {
                write!(f, "-")?;
                has_middle = true;
            }
            write!(f, "B")?;
        }
        if self.contains(Key::RightL) {
            if !has_middle {
                write!(f, "-")?;
                has_middle = true;
            }
            write!(f, "L")?;
        }
        if self.contains(Key::RightG) {
            if !has_middle {
                write!(f, "-")?;
                has_middle = true;
            }
            write!(f, "G")?;
        }
        if self.contains(Key::RightT) {
            if !has_middle {
                write!(f, "-")?;
                has_middle = true;
            }
            write!(f, "T")?;
        }
        if self.contains(Key::RightS) {
            if !has_middle {
                write!(f, "-")?;
                has_middle = true;
            }
            write!(f, "S")?;
        }
        if self.contains(Key::RightD) {
            if !has_middle {
                write!(f, "-")?;
                has_middle = true;
            }
            write!(f, "D")?;
        }
        if self.contains(Key::RightZ) {
            if !has_middle {
                write!(f, "-")?;
            }
            write!(f, "Z")?;
        }

        Ok(())
    }
}
