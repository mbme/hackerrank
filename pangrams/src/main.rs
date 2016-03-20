use std::io;
use std::collections::HashSet;

fn readline () -> String {
    let mut input_str = String::new();

    io::stdin().read_line(&mut input_str).ok().expect("read error");

    input_str
}

fn main() {
    let sentence = &readline();
    let mut char_set: HashSet<char> = HashSet::new();
    for c in 'a' as u8 .. 'z' as u8 {
        char_set.insert(c as char);
    }

    for c in sentence.chars() {
        if c.is_alphabetic() {
            char_set.remove(&c.to_lowercase().next().unwrap());
        }
    }

    println!("{}", if char_set.len() > 0 { "not pangram" } else { "pangram" });
}
