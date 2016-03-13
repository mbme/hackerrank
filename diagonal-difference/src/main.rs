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
    let n : u8 = must_parse(readline());

    let mut pd: i32 = 0;
    let mut sd: i32 = 0;
    for i in 0..n {
        let line: Vec<i32> = must_parse_arr(readline());
        pd += *line.get(i as usize).unwrap();
        sd += *line.get((n - 1 - i) as usize).unwrap();
    }

    println!("{}", (pd - sd).abs());
}
