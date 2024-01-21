#[derive(Debug)]
pub struct ParseKeyError {
    pub raw: String,
}

// https://www.foreui.com/articles/Key_Code_Table.htm
fn parse_key_code(raw: &str) -> Result<u32, ParseKeyError> {
    let code = match raw {
        "backspace" => 8,
        "tab" => 9,
        "enter" => 13,
        "shift" => 16,
        "ctrl" => 17,
        "alt" => 18,
        "pause" | "break" => 19,
        "caps" => 20,
        "esc" => 27,
        "pageup" => 33,
        "pagedown" => 34,
        "end" => 35,
        "home" => 36,
        "left" => 37,
        "up" => 38,
        "right" => 39,
        "down" => 40,
        "insert" | "ins" => 45,
        "delete" | "del" => 46,
        "equals" | "plus" => 61,
        "modifier" | "mod" => 91,
        "num0" => 96,
        "num1" => 97,
        "num2" => 98,
        "num3" => 99,
        "num4" => 100,
        "num5" => 101,
        "num6" => 102,
        "num7" => 103,
        "num8" => 104,
        "num9" => 105,
        "numasterisk" => 106,
        "numplus" => 107,
        "numminus" => 109,
        "numdot" => 110,
        "numslash" => 111,
        "f1" => 112,
        "f2" => 113,
        "f3" => 114,
        "f4" => 115,
        "f5" => 116,
        "f6" => 117,
        "f7" => 118,
        "f8" => 119,
        "f9" => 120,
        "f10" => 121,
        "f11" => 122,
        "f12" => 123,
        "numlock" => 144,
        "scrolllock" => 145,
        "leftcontrol" | "lctrl" => 162,
        "rightcontrol" | "rctrl" => 163,
        "comma" | "less" => 188,
        "period" | "greater" => 190,
        "forwardslash" | "fslash" | "questionmark" | "question" => 191,
        "backtick" | "grave" => 192,
        "leftsquarebracket" | "lsbracket" | "leftcurlybracket" | "lcbracket" => 219,
        "backslash" | "bslash" | "bar" => 220,
        "rightsquarebracket" | "rsbracket" | "rightcurlybracket" | "rcbracket" => 221,
        "apostrophe" | "quote" => 222,
        c if c.len() == 1 => c.chars().next().unwrap().to_ascii_uppercase() as u32,
        _ => {
            return Err(ParseKeyError { raw: raw.to_string() });
        }
    };

    Ok(code)
}

pub fn parse_key_combo(raw_combo: &str) -> Vec<u32> {
    raw_combo.split("+").into_iter()
        .map(|raw| parse_key_code(raw).expect("Failed to parse keycode"))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::parse::parse_key_combo;

    #[test]
    fn test_parse_key_combo_letter() {
        let raw_combo = "ctrl+w";
        let expected: Vec<u32> = vec![17, 87];
        let actual = parse_key_combo(raw_combo);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_key_combo_number() {
        let raw_combo = "alt+2";
        let expected: Vec<u32> = vec![18, 50];
        let actual = parse_key_combo(raw_combo);

        assert_eq!(expected, actual);
    }
}
