fn main() {
    assert_eq!(String::from("fl"), longest_common_prefix(vec!("flower".to_string(), "flow".to_string(), "flight".to_string())));
    assert_eq!(String::from("hello"), longest_common_prefix(vec!("hello".to_string())));
    assert_eq!(String::from(""), longest_common_prefix(vec!("".to_string(),"".to_string())));
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.len() == 1 {
        return String::from(&strs[0]);
    }

    let mut result = String::from("");

    let mut start = 0;

    'one: loop {
        if strs[0].len() == start {
            break 'one;
        }
        let current = String::from(&strs[0].as_str()[start..start+1]);

        for str in &strs {
            if str.len() == start {
                break 'one;
            }
            if str[start..start+1] != current {
                break 'one;
            }
        }
        result.push_str(&current);
        start += 1;
    }

    return result;
}
