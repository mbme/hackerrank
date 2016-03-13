use std::io;
use std::str::FromStr;

fn readline () -> String {
    let mut input_str = String::new();

    io::stdin().read_line(&mut input_str).ok().expect("read error");

    input_str
}

fn must_parse<T> (num_str: String) -> T where T: FromStr {
    num_str.trim().parse().ok().expect("parse error")
}

fn must_parse_arr<T> (s: String) -> Vec<T> where T: FromStr {
    s.split_whitespace().map(|item| must_parse(item.to_string())).collect()
}

fn main() {
    let n: usize = must_parse(readline());

    let nums: Vec<i32> = must_parse_arr(readline());

    assert_eq!(n, nums.len());

    let mut pos = 0;
    let mut neg = 0;
    let mut zero = 0;

    nums.iter().fold((), |_, &num| {
        if num > 0 {
            pos += 1;
        } else if num < 0 {
            neg += 1;
        } else {
            zero += 1;
        }
    });

    println!("{:.6}", pos as f32 / n as f32);
    println!("{:.6}", neg as f32 / n as f32);
    println!("{:.6}", zero as f32 / n as f32);
}
