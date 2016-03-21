use std::io;
use std::collections::HashMap;

fn readline () -> String {
    let mut input_str = String::new();

    io::stdin().read_line(&mut input_str).ok().expect("read error");

    let newlines: &[_] = &['\n', '\r'];
    input_str.trim_right_matches(newlines).to_string()
}

fn must_parse<T> (num_str: &str) -> T where T: std::str::FromStr {
    num_str.trim().parse().ok().expect("parse error")
}

fn sort_str (s: &str) -> String {
    let mut chars = s.chars().collect::<Vec<char>>();
    chars.sort();

    chars.into_iter().collect()
}

fn c (n: u32) -> u32 {
    if n < 2 {
        return 0;
    }

    return n * (n - 1) / 2;
}

fn main() {
    for _ in 0..must_parse(&readline()) {
        let s = &readline();

        let mut m = HashMap::new();

        for substr_len in 1..s.len() {
            for i in 0..s.len() - substr_len + 1 {
                let substr = sort_str(&s[i..i+substr_len]);
                let new_count = match m.get(&substr) {
                    Some(n) => n + 1,
                    None => 1,
                };
                m.insert(substr, new_count);
            }
        }

        println!("{}", m.iter().fold(0, |acc, (_, count)| acc + c(*count)));
    }
}
