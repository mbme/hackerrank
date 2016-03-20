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
        let mut counter = 0;
        let s = readline();
        let s = s.trim();
        let mut last_char = s.chars().next().unwrap();

        for c in s[1..].chars() {
            if c == last_char {
                counter += 1;
            }

            last_char = c;
        }

        println!("{}", counter);
    }
}
