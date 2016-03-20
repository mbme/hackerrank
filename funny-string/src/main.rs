use std::io;

fn readline () -> String {
    let mut input_str = String::new();

    io::stdin().read_line(&mut input_str).ok().expect("read error");

    input_str
}

fn must_parse<T> (num_str: &str) -> T where T: std::str::FromStr {
    num_str.trim().parse().ok().expect("parse error")
}

fn main() {
    for _ in 0..must_parse(&readline()) {
        let s: Vec<i16> = readline().trim().chars().map(|c| c as i16).collect();
        let s_rev: Vec<&i16> = s.iter().rev().collect();
        let mut failed = false;
        for j in 1..s.len() {
            let l = (s[j] - s[j - 1]).abs();
            let r = (s_rev[j] - s_rev[j - 1]).abs();
            if l != r {
                failed = true;
                break;
            }
        }

        println!("{}", if failed { "Not Funny" } else { "Funny" });
    }
}
