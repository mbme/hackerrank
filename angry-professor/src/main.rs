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
    let test_cases: u8 = must_parse(readline());

    for _ in 0..test_cases {
        let config: Vec<usize> = must_parse_arr(readline());
        let k = config.get(1).unwrap();

        let mut arrivals: Vec<i8> = must_parse_arr(readline());
        arrivals.retain(|&x| x <= 0); // leave only punctual students

        println!("{}", if arrivals.len() < *k { "YES" } else { "NO" });
    }
}
