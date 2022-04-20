pub fn encode(strs: Vec<String>) -> String {
    let mut buffer = String::new();

    for str in strs {
        buffer.push_str(&format!("{}{}", str.len(), str));
    }

    buffer
}

pub fn decode(str: String) -> Vec<String> {
    let mut strs_buffer = vec![];
    let mut str_buffer = String::new();

    let mut word_end_index = 0;

    for (i, c) in str.chars().enumerate() {
        if i < word_end_index {
            str_buffer.push(c);
        } else {
            let word_len = c.to_digit(10).unwrap() as usize;
            word_end_index = i + 1 + word_len;
        }

        if word_end_index as u32 - i as u32 == 1 {
            strs_buffer.push(str_buffer.clone());
            str_buffer.clear();
        }
    }

    strs_buffer
}

#[cfg(test)]
mod tests {
    use super::{decode, encode};

    #[test]
    fn example_1() {
        let strs = vec![
            "lint".to_owned(),
            "code".to_owned(),
            "love".to_owned(),
            "you".to_owned(),
        ];
        let str = "4lint4code4love3you".to_owned();
        assert_eq!(encode(strs.clone()), str.clone());
        assert_eq!(decode(str), strs);
    }

    #[test]
    fn example_2() {
        let strs = vec![
            "we".to_owned(),
            "say".to_owned(),
            ":".to_owned(),
            "yes".to_owned(),
        ];
        let str = "2we3say1:3yes".to_owned();
        assert_eq!(encode(strs.clone()), str.clone());
        assert_eq!(decode(str), strs);
    }

    #[test]
    fn example_3() {
        let strs = vec![
            "empty".to_owned(),
            "".to_owned(),
            "strings".to_owned(),
            "".to_owned(),
        ];
        let str = "5empty07strings0".to_owned();
        assert_eq!(encode(strs.clone()), str.clone());
        assert_eq!(decode(str), strs);
    }
}
