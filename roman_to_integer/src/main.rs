use std::collections::HashMap;

fn main() {
    roman_to_int("XIV".to_string());
}

pub fn roman_to_int(s: String) -> i32 {
    if s == "" {
        return 0;
    }

    let map = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);

    let letters: Vec<char> = s.chars().collect();

    let mut result: i32 = map[&letters[letters.len() - 1]];

    for (i, letter) in letters.iter().enumerate().rev() {
        if i == letters.len() - 1 {
            continue;
        } else if map[letter] < map[&letters[i + 1]] {
            result -= map[letter];
        } else {
            result += map[letter];
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use crate::roman_to_int;
    #[test]
    fn one() {
        assert_eq!(3, roman_to_int("III".to_string()))
    }

    #[test]
    fn two() {
        assert_eq!(2243, roman_to_int("MMCCXLIII".to_string()))
    }

    #[test]
    fn three() {
        assert_eq!(0, roman_to_int("".to_string()))
    }
}
