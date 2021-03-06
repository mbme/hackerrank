use std::io;

fn readline () -> String {
    let mut input_str = String::new();

    io::stdin().read_line(&mut input_str).ok().expect("read error");

    return input_str;
}

fn must_parse<T> (num_str: String) -> T where T: std::str::FromStr {
    num_str.trim().parse().ok().expect("parse error")
}

fn main() {
    let num_str_1 = readline();
    let num_str_2 = readline();

    let num_1 : i32 = must_parse(num_str_1);
    let num_2 : i32 = must_parse(num_str_2);

    println!("{}", num_1 + num_2);
}
