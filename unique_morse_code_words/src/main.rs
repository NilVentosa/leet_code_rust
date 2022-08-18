fn main() {
    assert_eq!(
        2,
        unique_morse_representations(vec![
            "gin".to_string(),
            "zen".to_string(),
            "gig".to_string(),
            "msg".to_string()
        ])
    );
}

pub fn unique_morse_representations(words: Vec<String>) -> i32 {
    use std::collections::HashMap;

    let letters = HashMap::from([
        ('a', ".-"),
        ('b', "-..."),
        ('c', "-.-."),
        ('d', "-.."),
        ('e', "."),
        ('f', "..-."),
        ('g', "--."),
        ('h', "...."),
        ('i', ".."),
        ('j', ".---"),
        ('k', "-.-"),
        ('l', ".-.."),
        ('m', "--"),
        ('n', "-."),
        ('o', "---"),
        ('p', ".--."),
        ('q', "--.-"),
        ('r', ".-."),
        ('s', "..."),
        ('t', "-"),
        ('u', "..-"),
        ('v', "...-"),
        ('w', ".--"),
        ('x', "-..-"),
        ('y', "-.--"),
        ('z', "--.."),
    ]);

    let mut result: HashMap<String, bool> = HashMap::new();

    for word in words {
        let mut word_in_morse = String::new();

        for c in word.chars() {
            match letters.get(&c) {
                Some(&morse) => word_in_morse.push_str(morse),
                None => println!("Nope"),
            }
        }
        result.insert(word_in_morse, true);
    }

    return result.len() as i32;
}
