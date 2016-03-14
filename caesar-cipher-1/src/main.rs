use std::io;

fn readline () -> String {
    let mut input_str = String::new();

    io::stdin().read_line(&mut input_str).ok().expect("read error");

    input_str.trim().to_string()
}

fn must_parse<T> (num_str: String) -> T where T: std::str::FromStr {
    num_str.trim().parse().ok().expect("parse error")
}

const ALPHABET_SIZE: u8 = 26;

fn main() {
    readline();
    let s = readline();
    let k: u8 = must_parse(readline());
    let k = k % ALPHABET_SIZE;

    let new_s = s.chars().map(|c| {
        if !c.is_alphabetic() {
            return c;
        }

        let mut code = c as u8 + k;
        let max_code = if c.is_uppercase() { 90 } else { 122 };

        if code > max_code {
            code -= ALPHABET_SIZE;
        }

        return code as char;
    }).collect::<String>();

    println!("{}", new_s);
}
