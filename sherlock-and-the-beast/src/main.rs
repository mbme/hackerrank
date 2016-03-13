use std::io;

fn readline () -> String {
    let mut input_str = String::new();

    io::stdin().read_line(&mut input_str).ok().expect("read error");

    input_str
}

fn must_parse<T> (num_str: String) -> T where T: std::str::FromStr {
    num_str.trim().parse().ok().expect("parse error")
}

fn string_of (c: char, n: u32) -> String {
    (0..n).map(|_| c).collect::<String>()
}

fn main() {
    for _ in 0..must_parse(readline()) {
        let n: u32 = must_parse(readline());

        if n < 3 {
            println!("-1");
            continue;
        }

        if n % 3 == 0 {
            println!("{}", string_of('5', n));
            continue;
        }

        let mut n5 = n;
        let mut n3 = 0;

        loop {
            n5 -= 1;
            n3 += 1;

            if n5 % 3 == 0 && n3 % 5 == 0 {
                println!("{}{}", string_of('5', n5), string_of('3', n3));
                break;
            }

            if n5 == 0 {
                println!("-1");
                break;
            }
        }

    }
}
