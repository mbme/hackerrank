use std::io;
use std::collections::HashMap;

fn readline () -> String {
    let mut input_str = String::new();

    io::stdin().read_line(&mut input_str).ok().expect("read error");

    let newlines: &[_] = &['\n', '\r'];
    input_str.trim_right_matches(newlines).to_string()
}

fn str_stats (s: &str) -> HashMap<char, u32> {
    let mut map = HashMap::new();

    for c in s.chars() {
        let new_count = match map.get(&c) {
            Some(n) => n + 1,
            None => 1,
        };
        map.insert(c, new_count);
    }

    map
}

fn main() {
    let mut map1 = str_stats(&readline());
    let mut map2 = str_stats(&readline());

    let mut counter = 0;

    for (c, count) in map1.iter_mut() {
        let n = match map2.get(&c) {
            Some(n) => *n,
            None => 0,
        };

        if *count > n {
            counter += *count - n;
            *count = n;
        } else if *count < n {
            map2.insert(*c, *count);
            counter += n - *count;
        }
    }

    for (c, count) in map2.iter_mut() {
        let n = match map1.get(&c) {
            Some(n) => *n,
            None => 0,
        };

        if *count > n {
            counter += *count - n;
            *count = n;
        } else if *count < n {
            map1.insert(*c, *count);
            counter += n - *count;
        }
    }

    println!("{}", counter);
}
