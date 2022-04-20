pub fn encode(strs: Vec<String>) -> String {
    let mut buffer = String::new();

    for str in strs {
        buffer.push_str(&format!("{}{}", str.len(), str));
    }

    buffer
}

pub fn decode(str: String) -> Vec<String> {
    let mut buffer = vec![];
    let mut i = 0;

    while i < str.len() {
        let start = i + 1;
        let decoded_str_len = str[i..start].parse::<usize>().unwrap();
        let decoded_str = str[start..start + decoded_str_len].to_owned();
        buffer.push(decoded_str);
        i = start + decoded_str_len;
    }

    buffer
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
