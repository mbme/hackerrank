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
    let config: Vec<u32> = must_parse_arr(readline());
    let widths: Vec<u8> = must_parse_arr(readline());

    for _ in 0..*config.get(1).unwrap() {
        let test_config: Vec<u32> = must_parse_arr(readline());
        let i = test_config[0];
        let j = test_config[1];

        let mut min_width = 3;
        for (pos, &width) in widths.iter().enumerate() {
            if (pos as u32) < i {
                continue;
            }

            if (pos as u32) > j {
                break;
            }

            if width < min_width {
                min_width = width;
            }
        }

        println!("{}", min_width);
    }
}
