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
        let line = readline();
        let nstr = line.trim();
        let n: u64 = must_parse(nstr.to_string());

        let mut counter = 0;
        nstr.chars().fold((), |_, ch| {
            let dig = ch.to_digit(10).unwrap() as u64;
            if dig > 0 && n % dig == 0 {
                counter += 1;
            }
        });

        println!("{}", counter);
    }
}
