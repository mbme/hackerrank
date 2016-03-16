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
    let actual: Vec<u32> = must_parse_arr(readline());
    let expected: Vec<u32> = must_parse_arr(readline());

    if actual[2] > expected[2] {
        println!("{}", 10000);
        return;
    }

    if actual[2] < expected[2] {
        println!("0");
        return;
    }

    if actual[1] > expected[1] {
        println!("{}", (actual[1] - expected[1]) * 500);
        return;
    }

    if actual[1] < expected[1] {
        println!("0");
        return;
    }

    if actual[0] > expected[0] {
        println!("{}", (actual[0] - expected[0]) * 15);
        return;
    }

    println!("0");
}
