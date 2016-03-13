use std::io;

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
        let cycles: u8 = must_parse(readline());

        let mut height = 1;
        for i in 0..cycles {
            if i % 2 == 0 {
                height *= 2;
            } else {
                height += 1;
            }
        }

        println!("{}", height);
    }
}
