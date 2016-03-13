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
    readline();

    let mut sticks: Vec<u32> = must_parse_arr(readline());
    sticks.sort();

    while sticks.len() > 0 {
        println!("{}", sticks.len());
        let min = sticks[0];
        sticks = sticks.iter().map(|x| x - min).filter(|&x| x > 0).collect();
    }
}
