use std::io;
use std::cmp::max;

fn readline () -> String {
    let mut input_str = String::new();

    io::stdin().read_line(&mut input_str).ok().expect("read error");

    input_str
}

fn must_parse<T> (num_str: &str) -> T where T: std::str::FromStr {
    num_str.trim().parse().ok().expect("parse error")
}

fn must_parse_arr<T> (s: &str) -> Vec<T> where T: std::str::FromStr {
    s.split_whitespace().map(|item| must_parse(item)).collect()
}

fn main() {
    for _ in 0..must_parse(&readline()) {
        readline();
        let arr: Vec<i32> = must_parse_arr(&readline());

        let mut global_max = arr[0];
        let mut local_max = arr[0];

        let mut abs_max = max(0, arr[0]);
        let mut all_negative = true;
        let mut max_val = arr[0];

        for (i, &x) in arr.iter().enumerate() {
            if x >= 0 {
                all_negative = false;
            }

            if i == 0 {
                continue;
            }

            max_val = max(max_val, x);

            if abs_max + x > abs_max {
                abs_max += x;
            }

            local_max = max(x, local_max + x);
            global_max = max(global_max, local_max);
        }

        if all_negative {
            abs_max = max_val;
        }

        println!("{} {}", global_max, abs_max);
    }
}
