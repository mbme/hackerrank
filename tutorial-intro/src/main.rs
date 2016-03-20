use std::io;

fn readline () -> String {
    let mut input_str = String::new();

    io::stdin().read_line(&mut input_str).ok().expect("read error");

    input_str
}

fn must_parse<T> (num_str: &str) -> T where T: std::str::FromStr {
    num_str.trim().parse().ok().expect("parse error")
}

fn must_parse_arr<T> (s: &str) -> Vec<T> where T: std::str::FromStr {
    s.split_whitespace().map(|item| must_parse(item)).collect()
}

fn main() {
    let v: i16 = must_parse(&readline());
    readline();
    let arr: Vec<i16> = must_parse_arr(&readline());

    for (i, &item) in arr.iter().enumerate() {
        if item == v {
            println!("{}", i);
            break;
        }
    }
}
