use std::io;
use std::collections::HashMap;

fn readline () -> String {
    let mut input_str = String::new();

    io::stdin().read_line(&mut input_str).ok().expect("read error");

    input_str
}

fn main() {
    let line = readline();
    let line = line.trim();

    let mut map = HashMap::new();
    for c in line.chars() {
        let new_count = match map.get(&c) {
            Some(&0) => 1,
            Some(n) => n - 1,
            None => 1,
        };
        map.insert(c, new_count);
    }
    let unmatched = map.iter().fold(0, |acc, (_, count)| acc + count);
    println!("{}", if unmatched > 1 { "NO" } else { "YES" });
}
