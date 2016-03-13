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

fn sqrt (a: u32) -> u32 {
    (a as f32).sqrt().ceil() as u32
}

fn main() {
    for _ in 0..must_parse(readline()) {
        let params: Vec<u32> = must_parse_arr(readline());
        let a = params.get(0).unwrap();
        let b = params.get(1).unwrap();

        let mut min = sqrt(*a);
        let mut counter = 0;
        loop {
            let pow = min.pow(2);

            if pow > *b {
                break;
            }

            if pow >= *a {
                counter += 1;
            }

            min += 1;
        }

        println!("{}", counter);
    }
}
