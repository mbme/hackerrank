use std::io;

type Matrix = Vec<String>;

fn readline () -> String {
    let mut input_str = String::new();

    io::stdin().read_line(&mut input_str).ok().expect("read error");

    input_str.trim().to_string()
}

fn must_parse<T> (num_str: String) -> T where T: std::str::FromStr {
    num_str.trim().parse().ok().expect("parse error")
}

fn must_parse_arr<T> (s: String) -> Vec<T> where T: std::str::FromStr {
    s.split_whitespace().map(|item| must_parse(item.to_string())).collect()
}

trait FindFrom {
    fn find_from (&self, m: &str, from: usize) -> bool;
}

impl FindFrom for str {
    fn find_from (&self, m: &str, at: usize) -> bool {
        self[at..].starts_with(m)
    }
}

fn read_matrix (rows: u32) -> Matrix {
    (0..rows).map(|_| readline()).collect::<_>()
}

fn check_pattern (m: &Matrix, p: &Matrix, initial_row: usize, match_pos: usize) -> bool {
    let mut j = 0;
    for i in initial_row..m.len() {
        let mrow: &str = &m[i];
        let prow: &str = &p[j];

        if !mrow.find_from(prow, match_pos) {
            return false;
        }

        j += 1;

        if j == p.len() {
            return true;
        }
    }

    return false;
}

fn find_from (s1: &str, s2: &str, from: usize) -> Option<usize> {
    s1[from..].find(s2)
}

fn find_pattern(m: &Matrix, p: &Matrix) -> bool {
    let max_start_row = m.len() - p.len() + 1;

    let prow: &str = &p[0];
    for i in 0..max_start_row {
        let mrow: &str = &m[i];

        let mut start_pos = 0;
        while let Some(match_pos) = find_from(mrow, prow, start_pos) {
            if check_pattern(m, p, i, start_pos + match_pos) {
                return true;
            }
            start_pos = start_pos + match_pos + 1;
        }
    }

    return false;
}

fn main() {
    for _ in 0..must_parse(readline()) {
        let config: Vec<u32> = must_parse_arr(readline());
        let rows = config[0];
        let g = read_matrix(rows);

        let pconfig: Vec<u32> = must_parse_arr(readline());
        let prows = pconfig[0];
        let pattern = read_matrix(prows);

        println!("{}", if find_pattern(&g, &pattern) { "YES" } else { "NO" });
    }
}
