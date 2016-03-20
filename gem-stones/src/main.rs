use std::io;
use std::collections::HashSet;

fn readline () -> String {
    let mut input_str = String::new();

    io::stdin().read_line(&mut input_str).ok().expect("read error");

    input_str
}

fn must_parse<T> (num_str: &str) -> T where T: std::str::FromStr {
    num_str.trim().parse().ok().expect("parse error")
}

fn main() {
    let n = must_parse(&readline());

    let mut set = HashSet::new();

    for i in 0..n {
        let s = readline();
        let s = s.trim();

        if i == 0 {
            for c in s.chars() {
                set.insert(c);
            }
            continue;
        }

        let mut new_set = HashSet::new();
        for c in s.chars() {
            new_set.insert(c);
        }

        set = set.intersection(&new_set).cloned().collect();
        if set.is_empty() {
            break;
        }
    }

    println!("{}", set.len());
}
