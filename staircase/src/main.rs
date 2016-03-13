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
    let n : usize = must_parse(readline());

    for i in 0..n {
        let mut s = String::with_capacity(n);
        for j in 0..n {
            if j < n - 1 - i {
                s.push(' ');
            } else {
                s.push('#');
            }
        }
        println!("{}", s);
    }
}
