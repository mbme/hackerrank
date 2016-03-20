use std::io;

fn readline () -> String {
    let mut input_str = String::new();

    io::stdin().read_line(&mut input_str).ok().expect("read error");

    input_str
}

fn read_digits_matrix (n: usize) -> Vec<Vec<char>> {
    (0..n).map(|_| readline().trim().chars().collect::<Vec<char>>()).collect()
}

fn must_parse<T> (num_str: &str) -> T where T: std::str::FromStr {
    num_str.trim().parse().ok().expect("parse error")
}

fn must_parse_arr<T> (s: &str) -> Vec<T> where T: std::str::FromStr {
    s.split_whitespace().map(|item| must_parse(item)).collect()
}

fn num_arr_to_string (n: &Vec<u64>) -> String {
    n.iter().map(|d| d.to_string()).collect::<Vec<String>>().join(" ")
}

fn string_of (c: char, n: u32) -> String {
    (0..n).map(|_| c).collect::<String>()
}

fn main() {
    
}
