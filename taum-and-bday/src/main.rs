use std::io;
use std::cmp::min;

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
    for _ in 0..must_parse(readline()) {
        let line1: Vec<u64> = must_parse_arr(readline());
        let black = line1[0];
        let white = line1[1];

        let line2: Vec<u64> = must_parse_arr(readline());
        let x = line2[0];
        let y = line2[1];
        let z = line2[2];

        let xm = min(x, y + z);
        let ym = min(y, x + z);

        println!("{}", black*xm + white*ym);
    }
}
