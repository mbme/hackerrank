use std::io;
use std::cmp::min;
use std::collections::HashMap;

fn readline () -> String {
    let mut input_str = String::new();

    io::stdin().read_line(&mut input_str).ok().expect("read error");

    let newlines: &[_] = &['\n', '\r'];
    input_str.trim_right_matches(newlines).to_string()
}

fn must_parse<T> (num_str: &str) -> T where T: std::str::FromStr {
    num_str.trim().parse().ok().expect("parse error")
}

fn sum_counters (map: HashMap<char, usize>) -> usize {
    map.iter().fold(0, |acc, (_, counter)| acc + counter)
}

fn main() {
    for _ in 0..must_parse(&readline()) {
        let s = &readline();
        if s.len() % 2 == 1 {
            println!("-1");
            continue;
        }

        let mut map1 = HashMap::new();
        let mut map2 = HashMap::new();

        for (i, c) in s.chars().enumerate() {
            let map = if i < s.len() / 2 { &mut map1 } else { &mut map2 };
            let new_count = match map.get(&c) {
                Some(n) => n + 1,
                None => 1,
            };
            map.insert(c, new_count);
        }

        for (c, count) in map1.iter_mut() {
            let n = match map2.get(&c) {
                Some(n) => *n,
                None => 0,
            };

            let shared_val = min(*count, n);
            *count -= shared_val;
            map2.insert(*c, n - shared_val);
        }

        for (c, count) in map2.iter_mut() {
            let n = match map1.get(&c) {
                Some(n) => *n,
                None => 0,
            };

            let shared_val = min(*count, n);
            *count -= shared_val;
            map1.insert(*c, n - shared_val);
        }

        println!("{}", (sum_counters(map1) + sum_counters(map2)) / 2);
    }
}
