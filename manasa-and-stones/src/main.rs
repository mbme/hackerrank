use std::io;
use std::collections::HashSet;

fn readline () -> String {
    let mut input_str = String::new();

    io::stdin().read_line(&mut input_str).ok().expect("read error");

    input_str
}

fn must_parse<T> (num_str: String) -> T where T: std::str::FromStr {
    num_str.trim().parse().ok().expect("parse error")
}

fn main() {
    for _ in 0..must_parse(readline()) {
        let n: usize = must_parse(readline());
        let a: usize = must_parse(readline());
        let b: usize = must_parse(readline());

        let mut sums = HashSet::new();
        sums.insert(a);
        sums.insert(b);

        for _ in 1..n-1 {
            let mut new_sums = HashSet::new();
            for val in &sums {
                new_sums.insert(val + a);
                new_sums.insert(val + b);
            }
            sums = new_sums;
        }

        let mut results = sums.iter().collect::<Vec<&usize>>();
        results.sort();
        println!("{}", results.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
    }
}
