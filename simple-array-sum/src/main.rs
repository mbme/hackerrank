use std::io;

fn readline () -> String {
    let mut input_str = String::new();

    io::stdin().read_line(&mut input_str).ok().expect("read error");

    input_str
}

fn must_parse<T> (num_str: String) -> T where T: std::str::FromStr {
    num_str.trim().parse().ok().expect("parse error")
}

fn must_parse_arr<T> (s: String) -> Vec<T> where T: std::str::FromStr {
    s.split_whitespace().map(|item| must_parse(item.to_string())).collect()
}

fn main() {
    let size : i32 = must_parse(readline());
    let items: Vec<i32> = must_parse_arr(readline());

    assert_eq!(size, items.len() as i32);

    println!("{}", items.iter().fold(0, |acc, &x| acc + x));
}
