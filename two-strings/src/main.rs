use std::io;
use std::collections::HashSet;

fn readline () -> String {
    let mut input_str = String::new();

    io::stdin().read_line(&mut input_str).ok().expect("read error");

    let newlines: &[_] = &['\n', '\r'];
    input_str.trim_right_matches(newlines).to_string()
}

fn must_parse<T> (num_str: &str) -> T where T: std::str::FromStr {
    num_str.trim().parse().ok().expect("parse error")
}

fn main() {
    for _ in 0..must_parse(&readline()) {
        let a = &readline().chars().collect::<HashSet<char>>();
        let b = &readline().chars().collect::<HashSet<char>>();

        println!("{}", if a.intersection(&b).count() > 0 { "YES" } else { "NO" });
    }
}
